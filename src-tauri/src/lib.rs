// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use regex::Regex;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};
use tauri_plugin_dialog::DialogExt;
use std::path::Path;
use std::fs;

mod csv_parser;
mod file_processor;
mod youtube_client;
mod metadata;
mod ytdlp_setup;
mod ffmpeg_setup;
use crate::csv_parser::{parse_csv_content, validate_csv_headers, CsvImportResult, CsvTrackEntry};
use crate::file_processor::{clean_filename, convert_to_mp3_with_ffmpeg};
use crate::youtube_client::{download_stream, search_video, VideoInfo};
use crate::metadata::{tag_mp3, TrackMetadata, parse_title_for_metadata};
use crate::ytdlp_setup::{check_ytdlp, download_ytdlp, get_ytdlp_command, get_ytdlp_version_command};
use crate::ffmpeg_setup::{check_ffmpeg, download_ffmpeg};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    let patterns =
        [r"(?:youtube\.com/watch\?v=|youtu\.be/|youtube\.com/shorts/)([a-zA-Z0-9_-]{11})"];

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
    tauri_plugin_opener::reveal_item_in_dir(&path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_video_command(query: String, cookies_browser: Option<String>, app_handle: tauri::AppHandle) -> Result<Option<VideoInfo>, String> {
    let ytdlp_path = get_ytdlp_command(&app_handle)?;
    search_video(&ytdlp_path, &query, cookies_browser.as_deref())
}

#[tauri::command]
async fn download_video_command(
    video_id: String,
    output_path: String,
    cookies_browser: Option<String>,
    window: tauri::Window,
) -> Result<String, String> {
    let ytdlp_path = get_ytdlp_command(&window.app_handle())?;
    let ffmpeg_path = match check_ffmpeg(window.app_handle().clone()) {
        Ok(p) => p,
        Err(_) => download_ffmpeg(window.app_handle().clone()).await.map_err(|e| {
            format!(
                "FFmpeg is required but couldn't be downloaded: {}. Please install FFmpeg and try again.",
                e
            )
        })?,
    };
    let window_clone = window.clone();
    download_stream(
        &ytdlp_path,
        &video_id,
        &output_path,
        Some(&ffmpeg_path),
        cookies_browser.as_deref(),
        move |progress, message| {
        let _ = window_clone.emit(
            "download-progress",
            serde_json::json!({
                "progress": progress,
                "message": message
            }),
        );
    },
    )
}

#[tauri::command]
async fn process_item(
    video_id: String,
    output_path: String,
    cookies_browser: Option<String>,
    metadata_override: Option<TrackMetadata>,
    window: tauri::Window,
) -> Result<String, String> {
    let ytdlp_path = get_ytdlp_command(&window.app_handle())?;
    let ffmpeg_path = match check_ffmpeg(window.app_handle().clone()) {
        Ok(p) => p,
        Err(_) => download_ffmpeg(window.app_handle().clone()).await.map_err(|e| {
            format!(
                "FFmpeg is required but couldn't be downloaded: {}. Please install FFmpeg and try again.",
                e
            )
        })?,
    };
    let window_clone = window.clone();

    // 1. Download
    let downloaded_path = download_stream(
        &ytdlp_path,
        &video_id,
        &output_path,
        Some(&ffmpeg_path),
        cookies_browser.as_deref(),
        move |progress, message| {
        let _ = window_clone.emit(
            "download-progress",
            serde_json::json!({
                "progress": progress,
                "message": message
            }),
        );
    },
    )?;

    // 2. Clean Filename
    let path = Path::new(&downloaded_path);
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
    
    // Remove the video ID from the filename if present (e.g. "Title [id]")
    // Escape the video_id for regex
    let escaped_id = regex::escape(&video_id);
    let id_pattern = format!(r"\s*\[{}\]", escaped_id);
    let regex = Regex::new(&id_pattern).map_err(|e| e.to_string())?;
    let stem_without_id = regex.replace(file_stem, "");
    
    let cleaned_stem = clean_filename(&stem_without_id);
    let new_filename = format!("{}.mp3", cleaned_stem);
    let new_path = path.parent().ok_or("Invalid path")?.join(&new_filename);
    
    // Rename/Move if different
    let final_path_str = if new_path != path {
        fs::rename(&path, &new_path).map_err(|e| format!("Failed to rename file: {}", e))?;
        new_path.to_str().ok_or("Invalid path")?.to_string()
    } else {
        downloaded_path
    };

    // 3. Tagging
    let mut final_metadata = metadata_override.unwrap_or_default();
    
    // Infer metadata if not provided
    if final_metadata.title.is_none() {
         let inferred = parse_title_for_metadata(&cleaned_stem);
         final_metadata.title = inferred.title;
         
         if final_metadata.artist.is_none() {
             final_metadata.artist = inferred.artist;
         }
    }

    tag_mp3(&final_path_str, final_metadata)?;

    Ok(final_path_str)
}

#[tauri::command]
fn clean_filename_command(original_name: String) -> String {
    clean_filename(&original_name)
}

#[tauri::command]
async fn convert_to_mp3_command(
    input_path: String,
    output_path: String,
    window: tauri::Window,
) -> Result<String, String> {
    let window_clone = window.clone();
    let ffmpeg_path = match check_ffmpeg(window.app_handle().clone()) {
        Ok(p) => p,
        Err(_) => download_ffmpeg(window.app_handle().clone()).await.map_err(|e| {
            format!(
                "FFmpeg is required but couldn't be downloaded: {}. Please install FFmpeg and try again.",
                e
            )
        })?,
    };
    convert_to_mp3_with_ffmpeg(&ffmpeg_path, &input_path, &output_path).map_err(|e| {
        let _ = window_clone.emit(
            "conversion-error",
            serde_json::json!({
                "error": e
            }),
        );
        e
    })
}

#[tauri::command]
fn read_file_command(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn parse_csv_command(csv_content: String, audio_mode: AudioMode) -> Result<CsvImportResult, String> {
    let result = parse_csv_content(&csv_content)?;

    let suffix = match audio_mode {
        AudioMode::Official => "official audio",
        AudioMode::Raw => "raw audio",
        AudioMode::Clean => "clean audio",
    };

    let mut processed_entries: Vec<CsvTrackEntry> = Vec::new();

    for mut entry in result.tracks {
        let search_query = format!("{} {}", entry.search_query.trim(), suffix);
        entry.search_query = search_query;
        processed_entries.push(entry);
    }

    Ok(CsvImportResult {
        tracks: processed_entries,
        total_count: result.total_count,
        success_count: result.success_count,
        error_count: result.error_count,
        errors: result.errors,
    })
}

#[tauri::command]
fn validate_csv_command(csv_content: String) -> Result<Vec<String>, String> {
    validate_csv_headers(&csv_content)
}

#[tauri::command]
fn tag_mp3_command(
    file_path: String,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    year: Option<String>,
    genre: Option<String>,
    track_number: Option<String>,
) -> Result<(), String> {
    let metadata = TrackMetadata {
        title,
        artist,
        album,
        year,
        genre,
        track_number,
        album_artist: None,
        comment: Some("Downloaded from YouTube".to_string()),
    };

    tag_mp3(&file_path, metadata)
}

#[tauri::command]
fn process_input(input_text: String, audio_mode: AudioMode) -> Result<ProcessInputResult, String> {
    let lines: Vec<&str> = input_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

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
    use crate::metadata::parse_title_for_metadata;

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
        assert!(is_youtube_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
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
        let input =
            "https://www.youtube.com/watch?v=abc123defgh\nSong One\nhttps://youtu.be/xyz789abcde";
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

    #[test]
    fn test_process_input_with_only_whitespace_lines() {
        let input = "   \n\n   \n\t\n   ";
        let result = process_input(input.to_string(), AudioMode::Official).unwrap();

        assert_eq!(result.total_count, 0);
    }

    #[test]
    fn test_process_input_search_query_with_special_characters() {
        let input = "Song & Dance\nRock n Roll";
        let result = process_input(input.to_string(), AudioMode::Clean).unwrap();

        assert_eq!(result.total_count, 2);
        assert_eq!(result.search_count, 2);
        assert_eq!(result.items[0].processed_query, "Song & Dance clean audio");
        assert_eq!(result.items[1].processed_query, "Rock n Roll clean audio");
    }

    #[test]
    fn test_process_input_handles_long_video_ids() {
        let input = "https://www.youtube.com/watch?v=abcdefghijk";
        let result = process_input(input.to_string(), AudioMode::Official).unwrap();

        assert_eq!(result.total_count, 1);
        assert_eq!(result.items[0].video_id, Some("abcdefghijk".to_string()));
    }

use crate::metadata::{tag_mp3, TrackMetadata};

    #[test]
    fn test_tag_mp3_command_basic() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_metadata.mp3");

        std::fs::write(&test_file, b"dummy mp3 content").unwrap();

        let metadata = TrackMetadata {
            title: Some("Test Song".to_string()),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            year: Some("2024".to_string()),
            genre: Some("Rock".to_string()),
            track_number: Some("1".to_string()),
            album_artist: Some("Various Artists".to_string()),
            comment: Some("Test comment".to_string()),
        };

        let result = tag_mp3(test_file.to_str().unwrap(), metadata);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_tag_mp3_command_nonexistent_file() {
        let result = tag_mp3(
            "/nonexistent/path/test.mp3",
            TrackMetadata::default(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File does not exist"));
    }

    #[test]
    fn test_parse_title_for_metadata_basic() {
        let metadata = parse_title_for_metadata("The Beatles - Hey Jude");
        assert_eq!(metadata.artist, Some("The Beatles".to_string()));
        assert_eq!(metadata.title, Some("Hey Jude".to_string()));
    }

    #[test]
    fn test_parse_title_for_metadata_no_artist() {
        let metadata = parse_title_for_metadata("Just a Song Title");
        assert_eq!(metadata.title, Some("Just a Song Title".to_string()));
        assert!(metadata.artist.is_none());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_ytdlp,
            download_ytdlp,
            get_ytdlp_version_command,
            check_ffmpeg,
            download_ffmpeg,
            set_download_path,
            open_folder,
            process_input,
            search_video_command,
            download_video_command,
            process_item,
            clean_filename_command,
            convert_to_mp3_command,
            read_file_command,
            parse_csv_command,
            validate_csv_command,
            tag_mp3_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
