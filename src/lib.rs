use std::os::raw::c_int;

pub use std::io::{Result, Error};

pub fn ffi_result(ret: c_int) -> Result<c_int>
{
    if ret < 0 {
        Err(Error::from_raw_os_error(-ret))
    } else {
        Ok(ret)
    }
}

#[macro_export]
macro_rules! ffi_try {
    ($e:expr) => ({
        $crate::ffi_result(unsafe{$e})
    })
}

pub mod login;