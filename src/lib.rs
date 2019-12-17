use std::os::raw::c_int;

pub use std::io::{Error, Result};

pub fn to_result(result: c_int) -> Result<c_int> {
    if result < 0 {
        return Err(Error::from_raw_os_error(-result));
    }

    Ok(result)
}

#[macro_export]
macro_rules! ffi_try {
    ($e:expr) => {{
        $crate::to_result(unsafe { $e })
    }};
}

pub mod login;
