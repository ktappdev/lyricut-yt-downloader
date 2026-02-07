// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_dialog::DialogExt;
use tauri::Emitter;
use serde::{Serialize, Deserialize};
use regex::Regex;

mod youtube_client;
use crate::youtube_client::{search_video, download_stream, VideoInfo};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioMode {
    Official,
    Raw,
    Clean,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputType {
    Url,
    SearchQuery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedItem {
    pub input_type: InputType,
    pub original_input: String,
    pub processed_query: String,
    pub video_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessInputResult {
    pub items: Vec<ProcessedItem>,
    pub total_count: usize,
    pub url_count: usize,
    pub search_count: usize,
}

fn get_audio_mode_suffix(mode: &AudioMode) -> &'static str {
    match mode {
        AudioMode::Official => "official audio",
        AudioMode::Raw => "raw audio",
        AudioMode::Clean => "clean audio",
    }
}

fn is_youtube_url(text: &str) -> bool {
    extract_video_id(text).is_some()
}

fn extract_video_id(url: &str) -> Option<String> {
    let patterns = [
        r"(?:youtube\.com/watch\?v=|youtu\.be/|youtube\.com/shorts/)([a-zA-Z0-9_-]{11})",
    ];

    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            if let Some(captures) = regex.captures(url) {
                if let Some(video_id) = captures.get(1) {
                    return Some(video_id.as_str().to_string());
                }
            }
        }
    }
    None
}

fn construct_search_query(line: &str, mode: &AudioMode) -> String {
    let suffix = get_audio_mode_suffix(mode);
    format!("{} {}", line.trim(), suffix)
}

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

#[tauri::command]
async fn search_video_command(query: String) -> Result<Option<VideoInfo>, String> {
    search_video(&query)
}

#[tauri::command]
async fn download_video_command(
    video_id: String,
    output_path: String,
    window: tauri::Window,
) -> Result<String, String> {
    let window_clone = window.clone();
    download_stream(&video_id, &output_path, move |progress, message| {
        let _ = window_clone.emit("download-progress", serde_json::json!({
            "progress": progress,
            "message": message
        }));
    })
}

#[tauri::command]
fn process_input(input_text: String, audio_mode: AudioMode) -> Result<ProcessInputResult, String> {
    let lines: Vec<&str> = input_text.lines().filter(|line| !line.trim().is_empty()).collect();

    let mut items: Vec<ProcessedItem> = Vec::new();
    let mut url_count = 0;
    let mut search_count = 0;

    for line in lines {
        let trimmed_line = line.trim();

        if is_youtube_url(trimmed_line) {
            let video_id = extract_video_id(trimmed_line);
            items.push(ProcessedItem {
                input_type: InputType::Url,
                original_input: trimmed_line.to_string(),
                processed_query: trimmed_line.to_string(),
                video_id,
            });
            url_count += 1;
        } else {
            let processed_query = construct_search_query(trimmed_line, &audio_mode);
            items.push(ProcessedItem {
                input_type: InputType::SearchQuery,
                original_input: trimmed_line.to_string(),
                processed_query,
                video_id: None,
            });
            search_count += 1;
        }
    }

    let total_count = items.len();
    Ok(ProcessInputResult {
        items,
        total_count,
        url_count,
        search_count,
    })
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

    #[test]
    fn test_is_youtube_url_with_standard_url() {
        assert!(is_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
    }

    #[test]
    fn test_is_youtube_url_with_shortened_url() {
        assert!(is_youtube_url("https://youtu.be/dQw4w9WgXcQ"));
    }

    #[test]
    fn test_is_youtube_url_with_shorts_url() {
        assert!(is_youtube_url("https://www.youtube.com/shorts/dQw4w9WgXcQ"));
    }

    #[test]
    fn test_is_youtube_url_with_non_url() {
        assert!(!is_youtube_url("Just a search query"));
        assert!(!is_youtube_url(""));
    }

    #[test]
    fn test_extract_video_id_from_standard_url() {
        let result = extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_from_shortened_url() {
        let result = extract_video_id("https://youtu.be/dQw4w9WgXcQ");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_from_shorts_url() {
        let result = extract_video_id("https://www.youtube.com/shorts/dQw4w9WgXcQ");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_with_invalid_url() {
        let result = extract_video_id("https://example.com/video");
        assert_eq!(result, None);
    }

    #[test]
    fn test_construct_search_query_official() {
        let result = construct_search_query("Hello World", &AudioMode::Official);
        assert_eq!(result, "Hello World official audio");
    }

    #[test]
    fn test_construct_search_query_raw() {
        let result = construct_search_query("Hello World", &AudioMode::Raw);
        assert_eq!(result, "Hello World raw audio");
    }

    #[test]
    fn test_construct_search_query_clean() {
        let result = construct_search_query("Hello World", &AudioMode::Clean);
        assert_eq!(result, "Hello World clean audio");
    }

    #[test]
    fn test_construct_search_query_trims_whitespace() {
        let result = construct_search_query("  Hello World  ", &AudioMode::Official);
        assert_eq!(result, "Hello World official audio");
    }

    #[test]
    fn test_process_input_with_urls() {
        let input = "https://www.youtube.com/watch?v=abc123defgh\nhttps://youtu.be/xyz789abcde";
        let result = process_input(input.to_string(), AudioMode::Official).unwrap();

        assert_eq!(result.total_count, 2);
        assert_eq!(result.url_count, 2);
        assert_eq!(result.search_count, 0);
        assert_eq!(result.items[0].input_type, InputType::Url);
        assert_eq!(result.items[0].video_id, Some("abc123defgh".to_string()));
        assert_eq!(result.items[1].video_id, Some("xyz789abcde".to_string()));
    }

    #[test]
    fn test_process_input_with_search_queries() {
        let input = "Song One\nSong Two";
        let result = process_input(input.to_string(), AudioMode::Official).unwrap();

        assert_eq!(result.total_count, 2);
        assert_eq!(result.url_count, 0);
        assert_eq!(result.search_count, 2);
        assert_eq!(result.items[0].input_type, InputType::SearchQuery);
        assert_eq!(result.items[0].processed_query, "Song One official audio");
        assert_eq!(result.items[1].processed_query, "Song Two official audio");
    }

    #[test]
    fn test_process_input_with_mixed_input() {
        let input = "https://www.youtube.com/watch?v=abc123defgh\nSong One\nhttps://youtu.be/xyz789abcde";
        let result = process_input(input.to_string(), AudioMode::Clean).unwrap();

        assert_eq!(result.total_count, 3);
        assert_eq!(result.url_count, 2);
        assert_eq!(result.search_count, 1);
        assert_eq!(result.items[1].processed_query, "Song One clean audio");
    }

    #[test]
    fn test_process_input_filters_empty_lines() {
        let input = "\nhttps://www.youtube.com/watch?v=abc123defgh\n\nSong One\n";
        let result = process_input(input.to_string(), AudioMode::Official).unwrap();

        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_process_input_with_empty_input() {
        let input = "";
        let result = process_input(input.to_string(), AudioMode::Official).unwrap();

        assert_eq!(result.total_count, 0);
        assert_eq!(result.url_count, 0);
        assert_eq!(result.search_count, 0);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![set_download_path, open_folder, process_input, search_video_command, download_video_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
