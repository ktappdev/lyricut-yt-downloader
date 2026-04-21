use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::{Emitter, Manager};

#[cfg(target_os = "windows")]
const YTDLP_FILENAME: &str = "yt-dlp.exe";
#[cfg(not(target_os = "windows"))]
const YTDLP_FILENAME: &str = "yt-dlp";

#[cfg(target_os = "windows")]
const YTDLP_DOWNLOAD_URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe";
#[cfg(not(target_os = "windows"))]
const YTDLP_DOWNLOAD_URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp";

pub fn get_ytdlp_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    
    Ok(data_dir.join(YTDLP_FILENAME))
}

#[tauri::command]
pub fn check_ytdlp(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = get_ytdlp_path(&app_handle)?;
    
    if path.exists() {
        Ok(path.to_string_lossy().to_string())
    } else {
        Err("yt-dlp not installed".to_string())
    }
}

pub fn get_ytdlp_version(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let path = get_ytdlp_path(app_handle)?;
    
    if !path.exists() {
        return Err("yt-dlp not installed".to_string());
    }
    
    let output = Command::new(&path)
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to run yt-dlp: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("yt-dlp exited with error: {}", output.status));
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tauri::command]
pub fn get_ytdlp_version_command(app_handle: tauri::AppHandle) -> Result<String, String> {
    get_ytdlp_version(&app_handle)
}

#[tauri::command]
pub async fn download_ytdlp(app_handle: tauri::AppHandle, force: Option<bool>) -> Result<String, String> {
    let path = get_ytdlp_path(&app_handle)?;
    let force = force.unwrap_or(false);
    
    if path.exists() {
        if force {
            let _ = fs::remove_file(&path);
            let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
                "status": "updating",
                "message": "Removing old yt-dlp version"
            }));
        } else {
            let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
                "status": "already_installed",
                "message": "yt-dlp already installed"
            }));
            return Ok(path.to_string_lossy().to_string());
        }
    }
    
    let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
        "status": "downloading",
        "message": "Starting download..."
    }));
    
    let response = reqwest::get(YTDLP_DOWNLOAD_URL)
        .await
        .map_err(|e| {
            let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
                "status": "error",
                "message": format!("Failed to connect: {}", e)
            }));
            format!("Failed to download yt-dlp: {}", e)
        })?;
    
    if !response.status().is_success() {
        let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
            "status": "error",
            "message": format!("HTTP error: {}", response.status())
        }));
        return Err(format!("Failed to download yt-dlp: HTTP {}", response.status()));
    }
    
    let total_size = response.content_length().unwrap_or(0);
    
    let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
        "status": "downloading",
        "message": format!("Downloading {} bytes...", total_size)
    }));
    
    let bytes = response
        .bytes()
        .await
        .map_err(|e| {
            let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
                "status": "error",
                "message": format!("Failed to read response: {}", e)
            }));
            format!("Failed to read response body: {}", e)
        })?;
    
    let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
        "status": "installing",
        "message": "Writing binary to disk..."
    }));
    
    fs::write(&path, &bytes)
        .map_err(|e| {
            let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
                "status": "error",
                "message": format!("Failed to write file: {}", e)
            }));
            format!("Failed to write yt-dlp binary: {}", e)
        })?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
            .map_err(|e| {
                let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to set permissions: {}", e)
                }));
                format!("Failed to set executable permissions: {}", e)
            })?;
    }
    
    let _ = app_handle.emit("ytdlp-progress", serde_json::json!({
        "status": "complete",
        "message": "yt-dlp installed successfully"
    }));
    
    Ok(path.to_string_lossy().to_string())
}

pub fn get_ytdlp_command(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let path = get_ytdlp_path(app_handle)?;
    
    if path.exists() {
        Ok(path.to_string_lossy().to_string())
    } else {
        Err("yt-dlp not installed. Please wait for it to download.".to_string())
    }
}
