use std::env;
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tauri::Manager;

fn candidate_filenames() -> &'static [&'static str] {
    if cfg!(windows) {
        &["ffmpeg.exe", "ffmpeg.bat", "ffmpeg.cmd"]
    } else {
        &["ffmpeg"]
    }
}

fn ffmpeg_filename() -> &'static str {
    if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    }
}

fn find_in_path(binary_names: &[&str]) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    for dir in env::split_paths(&path_var) {
        for name in binary_names {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(path) {
            return meta.permissions().mode() & 0o111 != 0;
        }
        false
    }
    #[cfg(not(unix))]
    {
        true
    }
}

fn ffmpeg_zip_url() -> Result<&'static str, String> {
    if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
        return Ok(
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v6.1/ffmpeg-6.1-win-64.zip",
        );
    }
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        return Ok(
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v6.1/ffmpeg-6.1-linux-64.zip",
        );
    }
    if cfg!(target_os = "linux") && cfg!(target_arch = "aarch64") {
        return Ok(
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v6.1/ffmpeg-6.1-linux-arm-64.zip",
        );
    }
    if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        return Ok(
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v6.1/ffmpeg-6.1-macos-64.zip",
        );
    }
    if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        return Ok(
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v6.1/ffmpeg-6.1-macos-64.zip",
        );
    }

    Err("Unsupported platform for automatic FFmpeg download".to_string())
}

fn get_app_ffmpeg_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    Ok(data_dir.join("ffmpeg").join("6.1").join(ffmpeg_filename()))
}

fn verify_ffmpeg_runnable(ffmpeg_path: &Path) -> Result<(), String> {
    let status = Command::new(ffmpeg_path)
        .args(["-version"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| "ffmpeg found but is not runnable".to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("ffmpeg found but is not runnable".to_string())
    }
}

fn find_ffmpeg(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let local_ffmpeg = get_app_ffmpeg_path(app_handle)?;
    if local_ffmpeg.is_file() && is_executable(&local_ffmpeg) {
        if verify_ffmpeg_runnable(&local_ffmpeg).is_ok() {
            return Ok(local_ffmpeg);
        }
    }

    if let Some(system_ffmpeg) = find_in_path(candidate_filenames()).filter(|p| is_executable(p)) {
        if verify_ffmpeg_runnable(&system_ffmpeg).is_ok() {
            return Ok(system_ffmpeg);
        }
    }

    Err("ffmpeg not found".to_string())
}

fn zip_entry_is_ffmpeg(entry_name: &str) -> bool {
    let normalized = entry_name.replace('\\', "/");
    normalized.ends_with(&format!("/{}", ffmpeg_filename())) || normalized == ffmpeg_filename()
}

async fn download_ffmpeg_zip() -> Result<Vec<u8>, String> {
    let url = ffmpeg_zip_url()?;
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download FFmpeg: {}", e))?;
    if !response.status().is_success() {
        return Err(format!(
            "Failed to download FFmpeg: HTTP {}",
            response.status()
        ));
    }
    response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read FFmpeg download: {}", e))
        .map(|b| b.to_vec())
}

fn extract_ffmpeg_from_zip(zip_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let cursor = Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor).map_err(|e| format!("Invalid FFmpeg zip: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to read FFmpeg zip: {}", e))?;
        if !file.is_file() {
            continue;
        }
        let name = file.name().to_string();
        if !zip_entry_is_ffmpeg(&name) {
            continue;
        }
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| format!("Failed to extract FFmpeg: {}", e))?;
        return Ok(buf);
    }

    Err("FFmpeg binary not found inside zip".to_string())
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    Ok(())
}

fn write_executable(path: &Path, bytes: &[u8]) -> Result<(), String> {
    ensure_parent_dir(path)?;
    fs::write(path, bytes).map_err(|e| format!("Failed to write FFmpeg binary: {}", e))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("Failed to set FFmpeg permissions: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn check_ffmpeg(app_handle: tauri::AppHandle) -> Result<String, String> {
    let ffmpeg_path = find_ffmpeg(&app_handle)?;
    Ok(ffmpeg_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn download_ffmpeg(app_handle: tauri::AppHandle) -> Result<String, String> {
    if let Ok(existing) = find_ffmpeg(&app_handle) {
        return Ok(existing.to_string_lossy().to_string());
    }

    let app_ffmpeg_path = get_app_ffmpeg_path(&app_handle)?;

    let zip_bytes = download_ffmpeg_zip().await?;
    let ffmpeg_bytes = extract_ffmpeg_from_zip(&zip_bytes)?;
    write_executable(&app_ffmpeg_path, &ffmpeg_bytes)?;

    verify_ffmpeg_runnable(&app_ffmpeg_path)?;
    Ok(app_ffmpeg_path.to_string_lossy().to_string())
}
