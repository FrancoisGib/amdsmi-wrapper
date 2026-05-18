use std::sync::Arc;

use amdsmi_sys::{
    AMDSMI_GPU_UUID_SIZE, amdsmi_board_info_t, amdsmi_memory_type_t, amdsmi_power_info_t,
    amdsmi_proc_info_t, amdsmi_processor_handle,
};

use crate::{AmdSmiInner, Result, amdsmi_unsafe, types::EnergyCount};

#[derive(Debug, Clone)]
pub struct Processor {
    pub(crate) inner: amdsmi_processor_handle,
    pub(crate) amdsmi: Arc<AmdSmiInner>,
}

impl Processor {
    pub fn get_uuid(&self) -> Result<String> {
        let mut uuid_length = AMDSMI_GPU_UUID_SIZE;
        let mut uuid_buf = [0; AMDSMI_GPU_UUID_SIZE as usize];

        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_gpu_device_uuid(
            self.inner,
            &mut uuid_length,
            uuid_buf.as_mut_ptr()
        ))?;

        let uuid = unsafe { std::ffi::CStr::from_ptr(uuid_buf.as_ptr()) };
        Ok(uuid.to_string_lossy().into_owned())
    }

    pub fn get_board_info(&self) -> Result<String> {
        let mut info: amdsmi_board_info_t = unsafe { std::mem::zeroed() };
        amdsmi_unsafe!(
            self.amdsmi
                .lib
                .amdsmi_get_gpu_board_info(self.inner, &mut info)
        )?;

        let product_name = unsafe {
            std::ffi::CStr::from_ptr(info.product_name.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        Ok(product_name)
    }

    pub fn get_energy_count(&self) -> Result<EnergyCount> {
        let mut energy_accumulator = 0;
        let mut counter_resolution = 0.0;
        let mut timestamp = 0;

        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_energy_count(
            self.inner,
            &mut energy_accumulator,
            &mut counter_resolution,
            &mut timestamp
        ))?;

        Ok(EnergyCount {
            energy_accumulator,
            counter_resolution,
            timestamp,
        })
    }

    pub fn get_vram_usage(&self) -> Result<u64> {
        let mut vram_usage: u64 = 0;

        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_gpu_memory_usage(
            self.inner,
            amdsmi_memory_type_t::AMDSMI_MEM_TYPE_VRAM,
            &mut vram_usage,
        ))?;

        Ok(vram_usage)
    }

    pub fn get_power(&self) -> Result<u32> {
        let mut info: amdsmi_power_info_t = unsafe { std::mem::zeroed() };
        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_power_info(self.inner, &mut info))?;
        Ok(info.average_socket_power)
    }

    pub fn get_gpu_process_list(&self) -> Result<Vec<amdsmi_proc_info_t>> {
        let mut nb_processes = 0;
        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_gpu_process_list(
            self.inner,
            &mut nb_processes,
            std::ptr::null_mut()
        ))?;

        let mut processes: Vec<amdsmi_proc_info_t> =
            vec![unsafe { std::mem::zeroed() }; nb_processes as usize];

        amdsmi_unsafe!(self.amdsmi.lib.amdsmi_get_gpu_process_list(
            self.inner,
            &mut nb_processes,
            processes.as_mut_ptr()
        ))?;

        processes.truncate(nb_processes as usize);

        Ok(processes)
    }
}

unsafe impl Send for Processor {}
unsafe impl Sync for Processor {}
