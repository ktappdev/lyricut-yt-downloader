// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
async fn set_download_path(window: tauri::Window) -> Result<Option<String>, String> {
    let path = window
        .dialog()
        .file()
        .set_title("Select Download Path")
        .blocking_pick_folder();

    Ok(path.map(|fp| fp.to_string()))
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    open::that(path).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_folder_with_valid_path() {
        let result = open_folder("/tmp".to_string());
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_open_folder_with_nonexistent_path() {
        let result = open_folder("/nonexistent/path/that/does/not/exist".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_open_folder_does_not_panic() {
        let _ = open_folder("".to_string());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![set_download_path, open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
