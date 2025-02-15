use std::{thread, time};
use sysinfo::Pid;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

macro_rules! as_ptr {
    ($value:expr) => {
        $value as *mut core::ffi::c_void
    };
}

pub fn get_process_path(process_id: u32) -> Option<String> {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    let pid = Pid::from_u32(process_id);
    let process = system.process(pid)?;
    let process_path = process.exe()?.to_str()?;
    println!("Process path is {process_path}");
    Some(process_path.to_string())
}

pub fn get_process_id_by_hwnd(hwnd: isize) -> Option<u32> {
    get_process_id_by_hwnd_recursive(hwnd, 0)
}

fn get_process_id_by_hwnd_recursive(hwnd: isize, pass: u8) -> Option<u32> {
    let mut pid  = 0;

    unsafe {
        GetWindowThreadProcessId(HWND(as_ptr!(hwnd)), Option::from(&raw mut pid));
    }

    let mut system = sysinfo::System::new();
    system.refresh_all();
    let pid = Pid::from_u32(pid);
    let process = system.process(pid)?;
    if process.name() == "ApplicationFrameHost" && pass < 1 {
        let sleep_time = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
        return get_process_id_by_hwnd_recursive(hwnd, pass + 1)
    }

    Some(pid.as_u32())
}

