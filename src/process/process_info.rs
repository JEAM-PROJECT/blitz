use sysinfo::{
    Components, Disks, Networks, System,
};
use sysinfo::{Pid, Signal};

pub fn get_processes_info() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .values()
        .take(5) // Tomar los dos primeros procesos
        .map(|process| {
            format!(
                "{:?} \nCPU {:.2}% \nRAM {} MB",
                process.name(),
                process.cpu_usage(),
                process.memory() / 1048576
            )
        })
        .collect()
}