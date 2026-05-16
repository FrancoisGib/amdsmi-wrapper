macro_rules! allowlist {
    ($($name:ident),* $(,)?) => {
        &[
            $(stringify!($name)),*
        ]
    };
}

const ALLOWLIST_STRUCTS: &[&str] = allowlist![
    processor_type_t,
    amdsmi_status_t,
    amdsmi_board_info_t,
    amdsmi_power_info_t,
    amdsmi_process_handle_t,
    amdsmi_proc_info_t,
    amdsmi_processor_handle,
    amdsmi_init_flags_t,
    amdsmi_memory_type_t,
    amdsmi_process_info_t,
];

const ALLOWLIST_FUNCTIONS: &[&str] = allowlist![
    amdsmi_get_socket_handles,
    amdsmi_get_socket_info,
    amdsmi_get_processor_info,
    amdsmi_get_energy_count,
    amdsmi_get_gpu_memory_usage,
    amdsmi_get_gpu_device_uuid,
    amdsmi_init,
    amdsmi_shut_down,
    amdsmi_get_gpu_board_info,
    amdsmi_get_processor_handles,
    amdsmi_get_gpu_compute_process_info_by_pid,
    amdsmi_get_gpu_process_list,
    amdsmi_get_lib_version,
    amdsmi_get_power_info,
];

const ALLOWLIST_CONSTANTS: &[&str] = allowlist![AMDSMI_MAX_STRING_LENGTH, AMDSMI_GPU_UUID_SIZE];