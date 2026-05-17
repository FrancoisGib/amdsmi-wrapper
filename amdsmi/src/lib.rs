use std::sync::Arc;

use amdsmi_sys::{AmdSmiLib, amdsmi_init_flags_t, amdsmi_version_t};

use crate::{error::AmdSmiError, types::Result, utils::find_amdsmi_path};

pub mod error;
mod processor;
mod socket;
pub mod types;

#[allow(clippy::macro_metavars_in_unsafe)]
mod utils;

pub use processor::Processor;
pub use socket::Socket;

pub struct AmdSmi {
    inner: Arc<AmdSmiInner>,
}

struct AmdSmiInner {
    lib: AmdSmiLib,
}

impl AmdSmi {
    pub fn init() -> Result<Self> {
        let lib_path = find_amdsmi_path()?;
        let lib = unsafe { AmdSmiLib::new(lib_path) }.map_err(|_| AmdSmiError::LibraryNotFound)?;

        amdsmi_unsafe!(lib.amdsmi_init(amdsmi_init_flags_t::AMDSMI_INIT_AMD_GPUS as u64))?;
        let inner = Arc::new(AmdSmiInner { lib });
        Ok(Self { inner })
    }

    pub fn get_lib_version(&self) -> Result<(u32, u32, u32)> {
        let mut version: amdsmi_version_t = unsafe { std::mem::zeroed() };
        amdsmi_unsafe!(self.inner.lib.amdsmi_get_lib_version(&mut version))?;
        Ok((version.major, version.minor, version.release))
    }

    pub fn get_socket_handles(&self) -> Result<Vec<Socket>> {
        let mut socket_count = 0;
        amdsmi_unsafe!(
            self.inner
                .lib
                .amdsmi_get_socket_handles(&mut socket_count, std::ptr::null_mut())
        )?;

        let mut socket_handles = vec![std::ptr::null_mut(); socket_count as usize];
        amdsmi_unsafe!(
            self.inner
                .lib
                .amdsmi_get_socket_handles(&mut socket_count, socket_handles.as_mut_ptr())
        )?;

        Ok(socket_handles
            .into_iter()
            .map(|handle| Socket {
                inner: handle,
                amdsmi: self.inner.clone(),
            })
            .collect())
    }
}

impl Drop for AmdSmiInner {
    fn drop(&mut self) {
        unsafe { self.lib.amdsmi_shut_down() };
    }
}
