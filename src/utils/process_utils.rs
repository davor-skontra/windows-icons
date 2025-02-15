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
            println!("Process path had application framehost in it.");
            let windows = get_hwnds_by_process_id(process_id);
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

struct WindowData {
    pid: u32,
    hwnd: HWND
}

fn get_hwnds_by_process_id(process_id: u32) -> Vec<HWND>{
    println!("I should be loooking for pid {process_id}");
    let mut window_data_list: Vec<WindowData> = Vec::new();

    // Pass the mutable reference to the list as LPARAM to store results
    unsafe {
        let result = EnumWindows(Some(enum_windows_callback), LPARAM(&mut window_data_list as *mut Vec<WindowData> as isize));
    }

    let hwnds = window_data_list
        .iter()
        .filter(|d| d.pid == process_id)
        .map(|d| d.hwnd)
        .collect();

    hwnds
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {
    let mut pid = 0;

    GetWindowThreadProcessId(hwnd, Option::from(std::ptr::addr_of_mut!(pid)));

    let window_data_list = &mut *(l_param.0 as *mut Vec<WindowData>);

    let data = WindowData {
        pid, hwnd
    };

    window_data_list.push(data);

    BOOL::from(true)
}



