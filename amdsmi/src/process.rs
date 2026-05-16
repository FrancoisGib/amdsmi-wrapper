use amdsmi_sys::{amdsmi_get_gpu_compute_process_info_by_pid, amdsmi_process_info_t};

use crate::{Result, amdsmi_unsafe};

pub fn get_process_info(pid: u32) -> Result<amdsmi_process_info_t> {
    let mut info: amdsmi_process_info_t = unsafe { std::mem::zeroed() };
    amdsmi_unsafe!(amdsmi_get_gpu_compute_process_info_by_pid(pid, &mut info))?;
    Ok(info)
}
