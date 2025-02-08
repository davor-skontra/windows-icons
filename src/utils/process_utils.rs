use sysinfo::{Pid, Process, System};

pub fn get_process_path(process_id: u32) -> Option<String> {
    println!("trying to get process path from id");
    let mut system = sysinfo::System::new();
    system.refresh_all();
    let pid = Pid::from_u32(process_id);
    let process = system.process(pid)?;
    let process_path = process.exe()?.to_str()?;
    println!("Process path is {process_path}");
    if process_path.ends_with("ApplicationFrameHost.exe") {
        let parent = process.parent();
        if parent.is_some() {
            let parent_process = system.process(parent?);
            let parent = &get_process_name(parent?.as_u32()).unwrap();
            let tasks = parent_process.unwrap().tasks();
            if (tasks.is_none()) {
                return Some(process_path.to_string());
            }
            for task in tasks? {
                let task = system.process(task.to_owned())?;
                let name = task.name().to_str()?;
                println!("host with sub p named: {name}");
            }
        }
    }

    Some(process_path.to_string())
}

pub fn get_process_name(process_id: u32) -> Option<String>{
    let mut system = sysinfo::System::new();
    system.refresh_all();
    let pid = Pid::from_u32(process_id);
    let process = system.process(pid)?;
    let process_name = process.name().to_str()?;
    println!("Process name is {process_name}");
    if process_name.ends_with("ApplicationFrameHost.exe") {
        let parent = process.parent();
        if parent.is_some() {
            let parent_process = system.process(parent?);
            let parent = &get_process_name(parent?.as_u32()).unwrap();
            let tasks = parent_process.unwrap().tasks();
            if (tasks.is_none()) {
                return Some(process_name.to_string());
            }
            for task in tasks? {
                let task = system.process(task.to_owned())?;
                let name = task.name().to_str()?;
                println!("host with sub p named: {name}");
            }
        }
    }



    Some(process_name.to_string())
}

