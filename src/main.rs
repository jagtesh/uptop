use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let mut system = sysinfo::System::new();

    system.refresh_all();

    for (pid, proc_) in system.get_process_list() {
        println!("{:6}\t{:<30}\t{:<10}", pid, proc_.name(), proc_.status());
    }
}
