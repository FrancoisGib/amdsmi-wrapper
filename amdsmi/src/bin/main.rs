use amdsmi::AmdSmi;

fn main() {
    let amdsmi = AmdSmi;
    amdsmi.init().unwrap();
    let (major, minor, patch) = amdsmi.get_lib_version().unwrap();
    println!("Version: {major}.{minor}.{patch}");

    let socket_handles = amdsmi.get_socket_handles().unwrap();
    println!("socket handles: {socket_handles:?}");
    for socket in socket_handles {
        println!("socket_info: {}", socket.get_socket_info().unwrap());
        let processor_handles = socket.get_processor_handles().unwrap();
        println!("processor handles: {processor_handles:?}");

        for processor in processor_handles {
            println!("uuid: {}", processor.get_uuid().unwrap());
            println!("product_name: {}", processor.get_board_info().unwrap());
            println!();
            println!("energy: {:?}", processor.get_energy_count().unwrap());
            println!("power: {:?}", processor.get_power().unwrap());
            println!("vram: {}", processor.get_vram_usage().unwrap());

            // println!("{:#?}", processor.get_gpu_process_list().unwrap());
        }

        println!();
    }
}
