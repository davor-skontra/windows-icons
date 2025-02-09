use sysinfo::Pid;

pub fn get_process_path(process_id: u32) -> Option<String> {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    let pid = Pid::from_u32(process_id);
    let process = system.process(pid)?;
    let process_path = process.exe()?.to_str()?;
    println!("Process path is {process_path}");
    Some(process_path.to_string())
}

