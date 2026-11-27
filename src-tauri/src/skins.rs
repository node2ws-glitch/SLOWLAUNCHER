use crate::paths::get_launcher_data_dir;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinInfo {
    pub id: String,
    pub file_name: String,
    pub path: String,
    pub active: bool,
}

fn skins_dir() -> PathBuf {
    get_launcher_data_dir().join("skins")
}

fn active_skin_path() -> PathBuf {
    get_launcher_data_dir().join("active-skin.txt")
}

fn safe_png_name(name: &str) -> String {
    let stem = Path::new(name)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("skin");
    let cleaned: String = stem
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
        .take(40)
        .collect();
    if cleaned.is_empty() { "skin".to_string() } else { cleaned }
}

fn validate_skin_png(path: &Path) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("Skin must be a valid PNG image.".to_string());
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    if width != 64 || (height != 64 && height != 32) {
        return Err("Minecraft skins must be 64x64 or legacy 64x32 PNG files.".to_string());
    }
    Ok(())
}

pub fn list_skins() -> Result<Vec<SkinInfo>, String> {
    let dir = skins_dir();
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let active = fs::read_to_string(active_skin_path()).unwrap_or_default();
    let mut skins = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("png") {
            continue;
        }
        let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string();
        skins.push(SkinInfo {
            id: file_name.clone(),
            file_name: file_name.clone(),
            path: path.to_string_lossy().to_string(),
            active: active.trim() == file_name,
        });
    }
    skins.sort_by(|a, b| a.file_name.to_lowercase().cmp(&b.file_name.to_lowercase()));
    Ok(skins)
}

pub fn import_skin(source_path: &str) -> Result<SkinInfo, String> {
    let source = Path::new(source_path);
    if !source.is_file() {
        return Err("Selected skin file does not exist.".to_string());
    }
    validate_skin_png(source)?;
    let dir = skins_dir();
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let stem = safe_png_name(source.file_name().and_then(|name| name.to_str()).unwrap_or("skin.png"));
    let mut destination = dir.join(format!("{stem}.png"));
    let mut suffix = 2;
    while destination.exists() {
        destination = dir.join(format!("{stem}-{suffix}.png"));
        suffix += 1;
    }
    fs::copy(source, &destination).map_err(|error| error.to_string())?;
    let file_name = destination.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string();
    Ok(SkinInfo {
        id: file_name.clone(),
        file_name,
        path: destination.to_string_lossy().to_string(),
        active: false,
    })
}

pub fn set_active_skin(file_name: &str) -> Result<(), String> {
    if file_name.contains('/') || file_name.contains('\\') || file_name.trim().is_empty() {
        return Err("Invalid skin selection.".to_string());
    }
    let path = skins_dir().join(file_name);
    if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("png") {
        return Err("Skin not found in your SlowClient skin library.".to_string());
    }
    let active_path = active_skin_path();
    if let Some(parent) = active_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(active_path, file_name).map_err(|error| error.to_string())
}

pub fn delete_skin(file_name: &str) -> Result<(), String> {
    if file_name.contains('/') || file_name.contains('\\') || file_name.trim().is_empty() {
        return Err("Invalid skin selection.".to_string());
    }
    let path = skins_dir().join(file_name);
    if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("png") {
        return Err("Skin not found in your SlowClient skin library.".to_string());
    }
    fs::remove_file(path).map_err(|error| error.to_string())?;
    if fs::read_to_string(active_skin_path()).unwrap_or_default().trim() == file_name {
        let _ = fs::remove_file(active_skin_path());
    }
    Ok(())
}
