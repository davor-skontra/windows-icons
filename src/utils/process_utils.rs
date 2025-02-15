use std::{ffi::OsString, os::windows::ffi::OsStringExt};
use std::collections::VecDeque;
use sysinfo::Pid;
use windows::core::w;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::System::ProcessStatus::K32EnumProcessModules;
use windows::Win32::UI::WindowsAndMessaging::{EnumChildWindows, EnumWindows, GetWindowThreadProcessId};

pub fn get_process_path(process_id: u32) -> Result<String, windows::core::Error> {
    unsafe {
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )?;

        let mut buffer = vec![0u16; 1024];
        let size = K32GetModuleFileNameExW(HANDLE(process_handle.0), None, &mut buffer);
        CloseHandle(process_handle)?;

        if size == 0 {
            return Err(windows::core::Error::from_win32());
        }

        buffer.truncate(size as usize);
        let path = OsString::from_wide(&buffer).into_string().map_err(|_| {
            windows::core::Error::new(
                windows::core::HRESULT(-1),
                "Invalid Unicode in path",
            )
        })?;
            let hwnds =
        if (&path).ends_with("ApplicationFrameHost.exe") {
            let mut system = sysinfo::System::new();
            system.refresh_all();
            let parent = system.process(Pid::from_u32(process_id)).unwrap().parent().unwrap().as_u32();
            println!("Process path had application framehost in it.");
            let windows = get_hwnds_by_process_id(parent);
            let length = windows.len();
            println!("Found windows nr {length}");
            for HWND(window ) in windows {
                let window = window as isize;
                println!("found window with hwnd {window}");
            }
        };

        Ok(path)
    }
}

fn get_hwnds_by_process_id(process_id: u32) -> Vec<HWND>{
    println!("I should be loooking for pid {process_id}");
    let mut hwnd_list: Vec<HWND> = Vec::new();

    // Pass the mutable reference to the list as LPARAM to store results
    unsafe {
        let result = EnumWindows(Some(enum_windows_callback), LPARAM(&mut hwnd_list as *mut Vec<HWND> as isize));
    }

    hwnd_list
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {
    let mut pid = 0;

    GetWindowThreadProcessId(hwnd, Some(&mut pid));
    let windows = &mut *(l_param.0 as *mut Vec<HWND>);
    windows.push(hwnd);
    println!("found pid {pid}");
    let l = l_param.0 as isize;
    println!("l param was {l}");
    let equal = pid == l as u32;

    println!("this was a match: {equal}");
    if equal {
        panic!("NICE!")
    }
    BOOL::from(equal)


}



