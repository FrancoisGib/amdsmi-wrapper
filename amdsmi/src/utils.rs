#[macro_export]
macro_rules! amdsmi_unsafe {
    ($fn:expr) => {{
        let status = unsafe { $fn };
        if status != amdsmi_sys::amdsmi_status_t::AMDSMI_STATUS_SUCCESS {
            Err($crate::error::AmdSmiError::from(status))
        } else {
            Ok(())
        }
    }};
}
