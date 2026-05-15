use amdsmi_sys::amdsmi_status_t;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmdSmiError {
    #[error("Driver not loaded")]
    DriverNotLoaded,

    #[error("Operation not supported")]
    NotSupported,

    #[error("amdsmi error, status: {0:?}")]
    AmdSmiError(amdsmi_status_t),
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
