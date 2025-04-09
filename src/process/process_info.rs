use sysinfo::{
    Components, Disks, Networks, System,
};

pub fn get_first_process_info() -> Option<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .values()
        .next()
        .map(|process| 
            format!(
                "{:?} \nCPU {:.2}% \nRAM {} GB", 
                process.name(), 
                process.cpu_usage(), 
                process.memory()/1048576))
}

pub fn get_first_two_processes_info() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .values()
        .take(5) // Tomar los dos primeros procesos
        .map(|process| {
            format!(
                "{:?} \nCPU {:.2}% \nRAM {} GB",
                process.name(),
                process.cpu_usage(),
                process.memory() / 1048576
            )
        })
        .collect()
}