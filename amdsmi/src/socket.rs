use std::sync::Arc;

use amdsmi_sys::{AMDSMI_MAX_STRING_LENGTH, amdsmi_socket_handle};

use crate::{AmdSmiInner, Result, amdsmi_unsafe, processor::Processor};

pub struct Socket {
    pub(crate) inner: amdsmi_socket_handle,
    pub(crate) amdsmi: Arc<AmdSmiInner>,
}

impl Socket {
    pub fn get_socket_info(&self) -> Result<String> {
        let mut name_buf = [0; AMDSMI_MAX_STRING_LENGTH as usize];
        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_socket_info(
            self.inner,
            AMDSMI_MAX_STRING_LENGTH as usize,
            name_buf.as_mut_ptr()
        ))?;

        let name = unsafe { std::ffi::CStr::from_ptr(name_buf.as_ptr()) };
        Ok(name.to_string_lossy().into_owned())
    }

    pub fn get_processor_handles(&self) -> Result<Vec<Processor>> {
        let mut processor_count = 0;
        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_processor_handles(
            self.inner,
            &mut processor_count,
            std::ptr::null_mut()
        ))?;

        let mut processor_handles = vec![std::ptr::null_mut(); processor_count as usize];
        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_processor_handles(
            self.inner,
            &mut processor_count,
            processor_handles.as_mut_ptr(),
        ))?;

        Ok(processor_handles
            .into_iter()
            .map(|handle| Processor {
                inner: handle,
                amdsmi: self.amdsmi.clone(),
            })
            .collect())
    }
}

unsafe impl Send for Socket {}
unsafe impl Sync for Socket {}
