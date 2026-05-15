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