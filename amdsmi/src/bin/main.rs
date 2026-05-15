use amdsmi::AmdSmi;

fn main() {
    let amdsmi = AmdSmi;
    amdsmi.init().unwrap();

    let socket_handles = amdsmi.get_socket_handles().unwrap();
    println!("socket handles: {socket_handles:?}");
    for socket in socket_handles {
        println!("socket_info: {}", socket.get_socket_info().unwrap());
        let processor_handles = socket.get_processor_handles().unwrap();
        println!("processor handles: {processor_handles:?}");

        for processor in processor_handles {
            println!("{}", processor.get_uuid().unwrap());
            println!("{:?}", processor.get_energy_count().unwrap());
            println!("{}", processor.get_board_info().unwrap());
            println!();
        }

        println!();
        println!();
    }
}
