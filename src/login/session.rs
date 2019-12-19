// https://www.freedesktop.org/software/systemd/man/sd-login.html

use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Result;
use std::os::raw::{c_char, c_void};
use std::ptr;

use log::trace;

use systemd_sys::login;

static SEAT0: &str = "seat0"; // "seat0" always exists.

#[derive(Debug)]
pub enum State {
    Unknown,
    Online,
    Active,
    Closing,
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        match s {
            "online" => State::Online,
            "active" => State::Active,
            "closing" => State::Closing,
            state @ _ => {
                trace!("unknown session state {}", state);
                State::Unknown
            }
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Unspecified,
    X11,
    Wayland,
    TTY,
    Mir,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "x11" => Type::X11,
            "wayland" => Type::Wayland,
            "tty" => Type::TTY,
            "mir" => Type::Mir,
            "unspecified" => Type::Unspecified,
            r#type @ _ => panic!("unknown session type {}", r#type),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Class {
    User,
    Greeter,
    LockScreen,
    Background,
}

impl From<&str> for Class {
    fn from(s: &str) -> Self {
        match s {
            "user" => Class::User,
            "greeter" => Class::Greeter,
            "lock-screen" => Class::LockScreen,
            "background" => Class::Background,
            class @ _ => panic!("unknown session class {}", class),
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub identifier: String,
    pub uid: u32,
}

impl Display for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.identifier, self.uid)
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Session {
    pub fn from_process_id(pid: i32) -> Result<Self> {
        let mut session_ptr: *mut c_char = ptr::null_mut();
        let _ = ffi_try!(login::sd_pid_get_session(pid, &mut session_ptr))?;

        let mut uid: u32 = 0;
        let _ = ffi_try!(login::sd_session_get_uid(session_ptr, &mut uid))?;

        let session: Session;
        unsafe {
            session = Session {
                identifier: CStr::from_ptr(session_ptr).to_string_lossy().to_string(),
                uid,
            };

            libc::free(session_ptr as *mut c_void);
        };

        Ok(session)
    }

    pub fn get_state(&self) -> Result<State> {
        let mut state_ptr: *mut c_char = ptr::null_mut();
        let _ = ffi_try!(login::sd_session_get_state(
            self.identifier.as_bytes().as_ptr() as *const i8,
            &mut state_ptr
        ))?;
        let state: State;
        unsafe {
            state = CStr::from_ptr(state_ptr)
                .to_string_lossy()
                .to_string()
                .as_str()
                .into();
            libc::free(state_ptr as *mut c_void);
        };

        Ok(state)
    }

    pub fn get_type(&self) -> Result<Type> {
        let mut type_ptr: *mut c_char = ptr::null_mut();
        let _ = ffi_try!(login::sd_session_get_type(
            self.identifier.as_bytes().as_ptr() as *const i8,
            &mut type_ptr
        ))?;
        let r#type: Type;
        unsafe {
            r#type = CStr::from_ptr(type_ptr)
                .to_string_lossy()
                .to_string()
                .as_str()
                .into();
            libc::free(type_ptr as *mut c_void);
        };

        Ok(r#type)
    }

    pub fn get_class(&self) -> Result<Class> {
        let mut class_ptr: *mut c_char = ptr::null_mut();
        let _ = ffi_try!(login::sd_session_get_class(
            self.identifier.as_bytes().as_ptr() as *const i8,
            &mut class_ptr
        ))?;
        let class: Class;
        unsafe {
            class = CStr::from_ptr(class_ptr)
                .to_string_lossy()
                .to_string()
                .as_str()
                .into();
            libc::free(class_ptr as *mut c_void);
        };

        Ok(class)
    }
}

pub fn get_active_session() -> Result<Session> {
    let seat = CString::new(SEAT0).unwrap();
    let mut session_ptr: *mut c_char = ptr::null_mut();
    let mut uid: u32 = 0;
    let session: Session;

    let _ = ffi_try!(login::sd_seat_get_active(
        seat.as_ptr(),
        &mut session_ptr,
        &mut uid
    ))?;

    unsafe {
        session = Session {
            identifier: CStr::from_ptr(session_ptr).to_string_lossy().to_string(),
            uid,
        };

        libc::free(session_ptr as *mut c_void);
    }

    Ok(session)
}

pub fn get_session(identifier: &str) -> Result<Option<Session>> {
    Ok(get_sessions()?
        .into_iter()
        .find(|session| session.identifier == identifier))
}

pub fn get_sessions() -> Result<Vec<Session>> {
    let seat = CString::new(SEAT0).unwrap();
    let mut sessions_ptr: *mut *mut c_char = ptr::null_mut();
    let mut uids_ptr: *mut u32 = ptr::null_mut();

    let num_sessions = ffi_try!(login::sd_seat_get_sessions(
        seat.as_ptr(),
        &mut sessions_ptr,
        &mut uids_ptr,
        ptr::null_mut()
    ))?;

    let mut sessions: Vec<Session> = Vec::with_capacity(num_sessions.try_into().unwrap());

    unsafe {
        for i in 0..num_sessions as isize {
            let session_ptr = *sessions_ptr.offset(i);
            let session = CStr::from_ptr(session_ptr);

            let uid_ptr = *uids_ptr.offset(i);

            sessions.push(Session {
                identifier: session.to_string_lossy().to_string(),
                uid: uid_ptr,
            });

            libc::free(session_ptr as *mut c_void);
        }

        libc::free(sessions_ptr as *mut c_void);
        libc::free(uids_ptr as *mut c_void);
    }

    Ok(sessions)
}
