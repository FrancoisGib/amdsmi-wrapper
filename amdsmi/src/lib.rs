use amdsmi_sys::{amdsmi_get_socket_handles, amdsmi_init, amdsmi_init_flags_t, amdsmi_shut_down, amdsmi_status_t};

use crate::{error::AmdSmiError, socket::Socket};

pub mod error;
pub mod socket;
pub mod processor;

#[macro_export]
macro_rules! amdsmi_unsafe {
    ($fn:expr) => {{
        let status = unsafe { $fn };
        if status != amdsmi_sys::amdsmi_status_t::AMDSMI_STATUS_SUCCESS {
            Err(crate::error::AmdSmiError::from(status))
        } else {
            Ok(())
        }
    }};
}

impl From<amdsmi_status_t> for AmdSmiError {
    fn from(status: amdsmi_status_t) -> Self {
        match status {
            amdsmi_status_t::AMDSMI_STATUS_DRIVER_NOT_LOADED => Self::DriverNotLoaded,
            amdsmi_status_t::AMDSMI_STATUS_NOT_SUPPORTED => Self::NotSupported,
            _ => Self::AmdSmiError(status),
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, AmdSmiError>;

pub struct AmdSmi;

impl AmdSmi {
    pub fn init(&self) -> Result<()> {
        amdsmi_unsafe!(amdsmi_init(
            amdsmi_init_flags_t::AMDSMI_INIT_AMD_GPUS as u64
        ))
    }

    pub fn get_socket_handles(&self) -> Result<Vec<Socket>> {
        let mut socket_count = 0;
        amdsmi_unsafe!(amdsmi_get_socket_handles(
            &mut socket_count,
            std::ptr::null_mut()
        ))?;

        let mut socket_handles = vec![std::ptr::null_mut(); socket_count as usize];
        amdsmi_unsafe!(amdsmi_get_socket_handles(
            &mut socket_count,
            socket_handles.as_mut_ptr()
        ))?;

        Ok(socket_handles
            .into_iter()
            .map(|handle| Socket { inner: handle })
            .collect())
    }
}

impl Drop for AmdSmi {
    fn drop(&mut self) {
        unsafe { amdsmi_shut_down() };
    }
}
