use amdsmi_sys::{
    AMDSMI_GPU_UUID_SIZE, amdsmi_board_info_t, amdsmi_get_energy_count, amdsmi_get_gpu_board_info,
    amdsmi_get_gpu_device_uuid, amdsmi_processor_handle,
};

use crate::{Result, amdsmi_unsafe};

#[derive(Debug)]
pub struct Processor {
    pub(crate) inner: amdsmi_processor_handle,
}

impl Processor {
    pub fn get_uuid(&self) -> Result<String> {
        let mut uuid_length = AMDSMI_GPU_UUID_SIZE;
        let mut uuid_buf = [0; AMDSMI_GPU_UUID_SIZE as usize];

        amdsmi_unsafe!(amdsmi_get_gpu_device_uuid(
            self.inner,
            &mut uuid_length,
            uuid_buf.as_mut_ptr()
        ))?;

        let uuid = unsafe { std::ffi::CStr::from_ptr(uuid_buf.as_ptr()) };
        Ok(uuid.to_string_lossy().into_owned())
    }

    pub fn get_board_info(&self) -> Result<String> {
        let mut info: amdsmi_board_info_t = unsafe { std::mem::zeroed() };
        amdsmi_unsafe!(amdsmi_get_gpu_board_info(self.inner, &mut info))?;

        let product_name = unsafe {
            std::ffi::CStr::from_ptr(info.product_name.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        println!("{info:?}");

        Ok(product_name)
    }

    pub fn get_energy_count(&self) -> Result<EnergyCount> {
        let mut energy_accumulator = 0;
        let mut counter_resolution = 0.0;
        let mut timestamp = 0;

        amdsmi_unsafe!(amdsmi_get_energy_count(
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
}

#[derive(Debug)]
pub struct EnergyCount {
    pub energy_accumulator: u64,
    pub counter_resolution: f32,
    pub timestamp: u64,
}
