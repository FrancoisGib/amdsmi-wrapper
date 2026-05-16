use amdsmi_sys::{
    amdsmi_get_lib_version, amdsmi_get_socket_handles, amdsmi_init, amdsmi_init_flags_t,
    amdsmi_shut_down, amdsmi_version_t,
};

use crate::{socket::Socket, types::Result};

pub mod error;
pub mod process;
pub mod processor;
pub mod socket;
pub mod types;

#[allow(clippy::macro_metavars_in_unsafe)]
mod utils;

#[derive(Debug)]
pub struct AmdSmi;

impl<'a> AmdSmi {
    pub fn init(&self) -> Result<()> {
        amdsmi_unsafe!(amdsmi_init(
            amdsmi_init_flags_t::AMDSMI_INIT_AMD_GPUS as u64
        ))
    }

    pub fn get_lib_version(&self) -> Result<(u32, u32, u32)> {
        let mut version: amdsmi_version_t = unsafe { std::mem::zeroed() };
        amdsmi_unsafe!(amdsmi_get_lib_version(&mut version))?;
        Ok((version.major, version.minor, version.release))
    }

    pub fn get_socket_handles(&'a self) -> Result<Vec<Socket<'a>>> {
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
            .map(|handle| Socket {
                inner: handle,
                _amdsmi: self,
            })
            .collect())
    }
}

impl Drop for AmdSmi {
    fn drop(&mut self) {
        unsafe { amdsmi_shut_down() };
    }
}
