use base64::Engine as _;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::{thread, time};
use base64::engine::general_purpose;
use image::RgbaImage;
use crate::utils::image_utils::{get_hicon, icon_to_image};
use crate::utils::process_utils::get_process_path;
use crate::uwp_apps::{get_uwp_icon, get_uwp_icon_base64};

mod utils {
    pub mod image_utils;
    pub mod process_utils;
}

mod uwp_apps;
pub struct IconMatcher {
    pub display_scale: i16,
}

impl Default for IconMatcher {
    fn default() -> Self {
        IconMatcher {
            display_scale: 100
        }
    }
}

enum AppType {
    Universal,
    Desktop,
    Other
}

fn get_app_type(path: &str) -> AppType {
    println!("Get app type by path for {path}");

    if path.ends_with("ApplicationFrameHost.exe"){
        return AppType::Universal
    }

    if path.contains("WindowsApps") {
        return AppType::Desktop
    }

    AppType::Other
}

pub fn get_icon_by_process_id(process_id: u32) -> Option<RgbaImage> {
    let icon_matcher = &IconMatcher::default();
    get_icon_by_process_id_matching(process_id, icon_matcher)
}

pub fn get_icon_by_process_id_matching(process_id: u32, icon_matcher: &IconMatcher) -> Option<RgbaImage> {
    let path = &get_process_path(process_id).ok()?;
    match get_app_type(path) {
        AppType::Universal => {get_afh_icon(process_id, path, icon_matcher)}
        AppType::Desktop => { get_uwp_icon(path, icon_matcher).ok() }
        AppType::Other => {get_icon_by_path(path)}
    }
}

fn get_afh_icon(process_id: u32, path: &str, icon_matcher: &IconMatcher) -> Option<RgbaImage> {
    println!("getting afh icon for {process_id}, {path}");
    let wait_millis = 1000;
    let mut total_wait_millis = 10000;
    let wait_time = time::Duration::from_millis(wait_millis);
    while get_process_path(process_id).ok()?.ends_with("ApplicationFrameHost.exe") {
        if total_wait_millis <= 0 {
            break;
        }
        thread::sleep(wait_time);
        total_wait_millis -= wait_millis;
    }

    let path = get_process_path(process_id).ok()?;
    get_uwp_icon(&path, icon_matcher).ok()
}

pub fn get_icon_by_path(path: &str) -> Option<RgbaImage> {
    unsafe {
        get_hicon(path).map(|icon| icon_to_image(icon).ok())?
    }
}

pub fn get_icon_base64_by_process_id(process_id: u32) -> Option<String> {
    let icon_matcher = &IconMatcher::default();
    if let Ok(path) = get_process_path(process_id) {
        get_icon_base64_by_path_matching(&path, icon_matcher)
    } else {
        None
    }
}

pub fn get_icon_base64_by_process_id_matching(process_id: u32, icon_matcher: &IconMatcher) -> Option<String> {
    if let Ok(path) = get_process_path(process_id) {
        get_icon_base64_by_path_matching(&path, icon_matcher)
    } else {
        None
    }
}

pub fn get_icon_base64_by_path(path: &str) -> Option<String>{
    let icon_matcher = &IconMatcher::default();
    get_icon_base64_by_path_matching(path, icon_matcher)
}

pub fn get_icon_base64_by_path_matching(path: &str, icon_matcher: &IconMatcher) -> Option<String> {
    match get_app_type(path) {
        AppType::Universal => {None}
        AppType::Desktop => {get_uwp_icon_base64(path, icon_matcher).ok()}
        AppType::Other => {
            if let Some(icon_image) = get_icon_by_path(path) {
                let mut buffer = Vec::new();
                icon_image
                    .write_to(
                        &mut std::io::Cursor::new(&mut buffer),
                        image::ImageFormat::Png,
                    ).ok();
                Some(general_purpose::STANDARD.encode(buffer))
            } else {
                None
            }
        }
    }
}



