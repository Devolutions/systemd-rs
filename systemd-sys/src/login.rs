use libc::{c_int, c_uint, c_char, pid_t, uid_t};

#[allow(non_camel_case_types)]
pub enum sd_login_monitor {}

extern "C" {
    pub fn sd_pid_get_session(pid: pid_t, session: *mut *mut c_char) -> c_int;
    pub fn sd_pid_get_owner_uid(pid: pid_t, uid: *mut uid_t) -> c_int;
    pub fn sd_pid_get_unit(pid: pid_t, unit: *mut *mut c_char) -> c_int;
    pub fn sd_pid_get_user_unit(pid: pid_t, unit: *mut *mut c_char) -> c_int;
    pub fn sd_pid_get_slice(pid: pid_t, slice: *mut *mut c_char) -> c_int;
    pub fn sd_pid_get_user_slice(pid: pid_t, slice: *mut *mut c_char) -> c_int;
    pub fn sd_pid_get_machine_name(pid: pid_t, machine: *mut *mut c_char) -> c_int;
    pub fn sd_pid_get_cgroup(pid: pid_t, cgroup: *mut *mut c_char) -> c_int;
    pub fn sd_peer_get_session(fd: c_int, session: *mut *mut c_char) -> c_int;
    pub fn sd_peer_get_owner_uid(fd: c_int, uid: *mut uid_t) -> c_int;
    pub fn sd_peer_get_unit(fd: c_int, unit: *mut *mut c_char) -> c_int;
    pub fn sd_peer_get_user_unit(fd: c_int, unit: *mut *mut c_char) -> c_int;
    pub fn sd_peer_get_slice(fd: c_int, slice: *mut *mut c_char) -> c_int;
    pub fn sd_peer_get_user_slice(fd: c_int, slice: *mut *mut c_char) -> c_int;
    pub fn sd_peer_get_machine_name(fd: c_int, machine: *mut *mut c_char) -> c_int;
    pub fn sd_peer_get_cgroup(pid: pid_t, cgroup: *mut *mut c_char) -> c_int;
    pub fn sd_uid_get_state(uid: uid_t, state: *mut *mut c_char) -> c_int;
    pub fn sd_uid_get_display(uid: uid_t, session: *mut *mut c_char) -> c_int;
    pub fn sd_uid_is_on_seat(uid: uid_t, require_active: c_int, seat: *const c_char) -> c_int;
    pub fn sd_uid_get_sessions(uid: uid_t,
                               require_active: c_int,
                               sessions: *mut *mut *mut c_char)
                               -> c_int;
    pub fn sd_uid_get_seats(uid: uid_t,
                            require_active: c_int,
                            seats: *mut *mut *mut c_char)
                            -> c_int;
    pub fn sd_session_is_active(session: *const c_char) -> c_int;
    pub fn sd_session_is_remote(session: *const c_char) -> c_int;
    pub fn sd_session_get_state(session: *const c_char, state: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_uid(session: *const c_char, uid: *mut uid_t) -> c_int;
    pub fn sd_session_get_seat(session: *const c_char, seat: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_service(session: *const c_char, service: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_type(session: *const c_char, _type: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_class(session: *const c_char, clazz: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_desktop(session: *const c_char, desktop: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_display(session: *const c_char, display: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_remote_host(session: *const c_char,
                                      remote_host: *mut *mut c_char)
                                      -> c_int;
    pub fn sd_session_get_remote_user(session: *const c_char,
                                      remote_user: *mut *mut c_char)
                                      -> c_int;
    pub fn sd_session_get_tty(session: *const c_char, display: *mut *mut c_char) -> c_int;
    pub fn sd_session_get_vt(session: *const c_char, vtnr: *mut c_uint) -> c_int;
    pub fn sd_seat_get_active(seat: *const c_char,
                              session: *mut *mut c_char,
                              uid: *mut uid_t)
                              -> c_int;
    pub fn sd_seat_get_sessions(seat: *const c_char,
                                sessions: *mut *mut *mut c_char,
                                uid: *mut *mut uid_t,
                                n_uids: *mut c_uint)
                                -> c_int;
    pub fn sd_seat_can_multi_session(seat: *const c_char) -> c_int;
    pub fn sd_seat_can_tty(seat: *const c_char) -> c_int;
    pub fn sd_seat_can_graphical(seat: *const c_char) -> c_int;
    pub fn sd_machine_get_class(machine: *const c_char, clazz: *mut *mut c_char) -> c_int;
    pub fn sd_machine_get_ifindices(machine: *const c_char, ifindices: *mut *mut c_int) -> c_int;
    pub fn sd_get_seats(seats: *mut *mut *mut c_char) -> c_int;
    pub fn sd_get_sessions(sessions: *mut *mut *mut c_char) -> c_int;
    pub fn sd_get_uids(users: *mut *mut uid_t) -> c_int;
    pub fn sd_get_machine_names(machines: *mut *mut *mut c_char) -> c_int;
    pub fn sd_login_monitor_new(category: *const c_char, ret: *mut *mut sd_login_monitor) -> c_int;
    pub fn sd_login_monitor_unref(m: *mut sd_login_monitor) -> *mut sd_login_monitor;
    pub fn sd_login_monitor_flush(m: *mut sd_login_monitor) -> c_int;
    pub fn sd_login_monitor_get_fd(m: *mut sd_login_monitor) -> c_int;
    pub fn sd_login_monitor_get_events(m: *mut sd_login_monitor) -> c_int;
    pub fn sd_login_monitor_get_timeout(m: *mut sd_login_monitor,
                                        timeout_usec: *mut u64)
                                        -> c_int;
}