use std::{error::Error, fs, path::Path};
use std::path::PathBuf;
use image::RgbaImage;
use crate::utils::image_utils::{get_icon_from_base64, read_image_to_base64};

pub fn get_uwp_icon(process_path: &str) -> Result<RgbaImage, Box<dyn Error>> {
    let icon_path = &get_icon_file_path(process_path)?;
    let exists = Path::exists(icon_path.as_ref());
    let base64 = read_image_to_base64(icon_path)?;
    let icon = get_icon_from_base64(&base64)?;
    Ok(icon)
}

pub fn get_uwp_icon_base64(process_path: &str) -> Result<String, Box<dyn Error>> {
    let icon_path = get_icon_file_path(process_path)?;
    let base64 = read_image_to_base64(&icon_path)?;
    Ok(base64)
}

fn get_icon_file_path(app_path: &str) -> Result<String, Box<dyn Error>> {
    let package_folder = Path::new(app_path).parent().unwrap();

    let desktop_icon_path = package_folder.join("assets").join("DesktopShortcut.ico");
    if desktop_icon_path.exists() {
        return Ok(desktop_icon_path.to_str().unwrap().to_string());
    } else {
        let manifest_path = package_folder.join("AppxManifest.xml");
        let manifest_content = fs::read_to_string(&manifest_path)?;
        let icon_path = extract_icon_path(&manifest_content)?;
        let icon_full_path = package_folder.join(icon_path).to_str().unwrap().to_string();
        let icon_scale_path = get_scaled_icon_path(&icon_full_path).unwrap_or(icon_full_path.to_string());
        return Ok(icon_scale_path);
    }
}

fn extract_icon_path(manifest_content: &str) -> Result<String, Box<dyn Error>> {
    // Look for the <Logo>...</Logo> tag in the manifest
    let tag = "Logo";
    let start_tag = &format!("<{tag}>");
    let end_tag = &format!("</{tag}>");

    if let Some(start) = manifest_content.find(start_tag) {
        if let Some(end) = manifest_content.find(end_tag) {
            let start_pos = start + start_tag.len();
            let icon_path = &manifest_content[start_pos..end];
            return Ok(icon_path.trim().to_string());
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Icon path not found in manifest.",
    )))
}

fn get_scaled_icon_path(icon_path: &str) -> Option<String> {
    let path = Path::new(icon_path);
    if path.is_dir() {
        return None
    }
    let folder_path = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let matching_files: Vec<PathBuf> = folder_path.read_dir().ok()?
        .filter_map(|de| de.ok())
        .map(|de| de.path())
        .filter(|de| de.is_file())
        .filter(|p| p.file_stem().unwrap().to_str().unwrap().contains(file_stem))
        .collect();

    if matching_files.is_empty() {
        return None
    }

    let first = matching_files[0].as_path();

    Some(first.to_str().unwrap().to_string())
}
