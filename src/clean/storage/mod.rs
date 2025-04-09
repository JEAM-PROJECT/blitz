use sysinfo::{Disk, DiskExt, System, SystemExt};

const BYTES_TO_GB: u64 = 1024 * 1024 * 1024;

pub fn main() {
    let sys = System::new_all();

    sys.refresh_all();

    let disk = Disks::new_with_refreshed_list();
}
