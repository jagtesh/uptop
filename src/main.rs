use sysinfo::{ProcessExt, SystemExt};

fn format_process_list(pid: &i32, proc: &sysinfo::Process) -> String {
    return format!("{:6}\t{:<30}\t{:<10}", pid, proc.name(), proc.status());
}

fn main() {
    let mut system = sysinfo::System::new();

    system.refresh_all();

    for (pid, proc) in system.get_process_list() {
        println!("{}", format_process_list(pid, proc));
    }
}
