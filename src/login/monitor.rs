use nix::errno::Errno;
use std::cell::Cell;
use std::convert::TryInto;
use std::io::Result;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::thread;

use epoll::{ControlOptions::EPOLL_CTL_ADD, Event, Events};
use systemd_sys::login;

pub enum Category {
    All,
    Seats,
    Sessions,
    Uids,
    Machines,
}

impl Category {
    fn as_str(&self) -> Option<&str> {
        match self {
            Category::All => None,
            Category::Seats => Some("seat"),
            Category::Sessions => Some("session"),
            Category::Uids => Some("uid"),
            Category::Machines => Some("machine"),
        }
    }
}

pub struct Monitor {
    handle: Cell<Option<thread::JoinHandle<()>>>,
    pipe_fds: [i32; 2],
}

impl Drop for Monitor {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            unsafe {
                libc::write(self.pipe_fds[1], [0x0A].as_ptr() as *const c_void, 1);
            }
            let _ = handle.join();
        }

        for fd in self.pipe_fds.iter() {
            if *fd > -1 {
                unsafe {
                    libc::close(*fd);
                }
            }
        }
    }
}

impl Monitor {
    pub fn new() -> Result<Self> {
        let mut pipe_fds = [-1; 2];
        ffi_try!(libc::pipe2(pipe_fds.as_mut_ptr(), libc::O_CLOEXEC))?;

        Ok(Monitor {
            handle: Cell::new(None),
            pipe_fds,
        })
    }

    pub fn init<F>(&self, category: Category, mut callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        let read_fd = self.pipe_fds[0];

        self.handle.set(Some(thread::spawn(move || {
            let category_cstr: std::ffi::CString;
            let category: *const c_char = match category.as_str() {
                Some(s) => {
                    category_cstr = std::ffi::CString::new(s).unwrap();
                    category_cstr.as_ptr()
                }
                None => ptr::null(),
            };

            let mut monitor: *mut login::sd_login_monitor = ptr::null_mut();
            ffi_try!(login::sd_login_monitor_new(category, &mut monitor)).unwrap();

            let ep_fd = epoll::create(true).unwrap();

            // Add the read fd from our self-pipe to epoll
            let mut events = Events::empty();
            events.insert(Events::EPOLLIN);
            let event = Event::new(events, read_fd.try_into().unwrap());
            epoll::ctl(ep_fd, EPOLL_CTL_ADD, read_fd, event).unwrap();

            // Add the events from sd_login_monitor to epoll
            let monitor_events = ffi_try!(login::sd_login_monitor_get_events(monitor)).unwrap();
            let monitor_fd = ffi_try!(login::sd_login_monitor_get_fd(monitor)).unwrap();

            let mut events = Events::from_bits(monitor_events.try_into().unwrap()).unwrap();
            events.insert(epoll::Events::EPOLLET);
            let event = Event::new(events, monitor_fd.try_into().unwrap());
            epoll::ctl(ep_fd, EPOLL_CTL_ADD, monitor_fd, event).unwrap();

            let read_fd: u64 = read_fd.try_into().unwrap();
            let monitor_fd: u64 = monitor_fd.try_into().unwrap();

            loop {
                let timeout = get_timeout(monitor).unwrap();
                let mut signalled = false;
                let mut events: [epoll::Event; 1024] = [epoll::Event { events: 0, data: 0 }; 1024];

                let num_fds = loop {
                    let wait_result = epoll::wait(ep_fd, timeout, &mut events);
                    match wait_result {
                        Ok(num) => break num,
                        Err(e) if Errno::last() != Errno::EINTR => {
                            panic!("Failure calling epoll_wait: {}", e);
                        }
                        Err(_) => {}
                    }
                };

                for i in 0..num_fds {
                    let fd = events[i].data;

                    if fd == read_fd {
                        signalled = true;
                        break;
                    } else if fd == monitor_fd {
                        callback();
                        ffi_try!(login::sd_login_monitor_flush(monitor)).unwrap();
                    }
                }

                if signalled {
                    break;
                }
            }

            unsafe {
                login::sd_login_monitor_unref(monitor);
            }
        })));

        Ok(())
    }
}

fn get_timeout(monitor: *mut login::sd_login_monitor) -> Result<i32> {
    let mut t: u64 = 0;
    ffi_try!(login::sd_login_monitor_get_timeout(monitor, &mut t))?;

    match t {
        std::u64::MAX => Ok(-1),
        _ => {
            let mut ts = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            let ts_ptr: *mut libc::timespec = &mut ts as *mut _ as *mut libc::timespec;

            unsafe {
                libc::clock_gettime(libc::CLOCK_MONOTONIC, ts_ptr);
            }

            let n: u64 = (ts.tv_sec * 1000000 + ts.tv_nsec / 1000)
                .try_into()
                .unwrap();
            let msec: i32 = if t > n {
                ((t - n + 999) / 1000).try_into().unwrap()
            } else {
                0
            };

            Ok(msec)
        }
    }
}
