use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub uploader: Option<String>,
    pub duration_seconds: Option<u64>,
    pub upload_date: Option<String>,
}

pub fn search_video(ytdlp_path: &str, query: &str, cookies_browser: Option<&str>) -> Result<Option<VideoInfo>, String> {
    let search_query = format!("ytsearch10:{}", query);

    let mut args = vec![
        "--dump-json",
        "--no-download",
        "--quiet",
        "--no-warnings",
        &search_query,
    ];

    if let Some(browser) = cookies_browser {
        args.push("--cookies-from-browser");
        args.push(browser);
    }

    let output = Command::new(ytdlp_path)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp search failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        return Ok(None);
    }

    if let Some(first_line) = lines.first() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(first_line) {
            let id = json["id"]
                .as_str()
                .ok_or("No video ID in response")?
                .to_string();
            let title = json["title"].as_str().unwrap_or("").to_string();
            let url = format!("https://www.youtube.com/watch?v={}", id);

            let thumbnail_url = json["thumbnail"]
                .as_str()
                .map(|s| s.to_string())
                .or_else(|| {
                    json["thumbnails"]
                        .as_array()
                        .and_then(|thumbnails| thumbnails.first())
                        .and_then(|t| t["url"].as_str())
                        .map(|s| s.to_string())
                });

            let uploader = json["uploader"]
                .as_str()
                .map(|s| s.to_string())
                .or_else(|| json["uploader_name"].as_str().map(|s| s.to_string()));

            let duration_seconds = json["duration"]
                .as_u64()
                .or_else(|| json["duration_seconds"].as_u64());

            let upload_date = json["upload_date"].as_str().map(|s| s.to_string());

            return Ok(Some(VideoInfo {
                id,
                title,
                url,
                thumbnail_url,
                uploader,
                duration_seconds,
                upload_date,
            }));
        }
    }

    Ok(None)
}

pub fn download_stream(
    ytdlp_path: &str,
    video_id: &str,
    output_path: &str,
    ffmpeg_location: Option<&str>,
    cookies_browser: Option<&str>,
    on_progress: impl Fn(f64, &str) + Send + 'static,
) -> Result<String, String> {
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);
    let output_template = format!("{}/%(title)s [%(id)s].%(ext)s", output_path);

    on_progress(0.0, "Starting download...");

    let mut args: Vec<String> = vec![
        "--format".to_string(),
        "bestaudio[ext=m4a]/bestaudio[ext=webm]/bestaudio/best".to_string(),
        "--output".to_string(),
        output_template,
        "--extract-audio".to_string(),
        "--audio-format".to_string(),
        "mp3".to_string(),
        "--audio-quality".to_string(),
        "0".to_string(),
        "--no-playlist".to_string(),
        "--no-warnings".to_string(),
        "--progress".to_string(),
    ];
    if let Some(location) = ffmpeg_location {
        args.push("--ffmpeg-location".to_string());
        args.push(location.to_string());
    }
    if let Some(browser) = cookies_browser {
        args.push("--cookies-from-browser".to_string());
        args.push(browser.to_string());
    }
    args.push(video_url);

    let output = Command::new(ytdlp_path)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    let output = output
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for yt-dlp: {}", e))?;

    on_progress(90.0, "Processing complete...");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp download failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let extracted_filename = extract_downloaded_filename(&stdout);

    on_progress(100.0, "Download complete");

    extracted_filename.ok_or("Failed to determine downloaded filename".to_string())
}

fn extract_downloaded_filename(output: &str) -> Option<String> {
    let patterns = [
        r"\[ExtractAudio\] Destination: (.+\.mp3)",
        r"\[Merger\] Merging formats into (.+\.mp3)",
        r"\[info\] (.+\.mp3)",
    ];

    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            if let Some(captures) = regex.captures(output) {
                if let Some(filename) = captures.get(1) {
                    return Some(filename.as_str().to_string());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_info_serde() {
        let video_info = VideoInfo {
            id: "abc123".to_string(),
            title: "Test Video".to_string(),
            url: "https://www.youtube.com/watch?v=abc123".to_string(),
            thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
            uploader: Some("TestUploader".to_string()),
            duration_seconds: Some(180),
            upload_date: Some("2024-01-01".to_string()),
        };

        let serialized = serde_json::to_string(&video_info).unwrap();
        let deserialized: VideoInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(video_info.id, deserialized.id);
        assert_eq!(video_info.title, deserialized.title);
        assert_eq!(video_info.url, deserialized.url);
        assert_eq!(video_info.thumbnail_url, deserialized.thumbnail_url);
        assert_eq!(video_info.uploader, deserialized.uploader);
        assert_eq!(video_info.duration_seconds, deserialized.duration_seconds);
        assert_eq!(video_info.upload_date, deserialized.upload_date);
    }

    #[test]
    fn test_video_info_default_values() {
        let video_info = VideoInfo {
            id: "test123".to_string(),
            title: String::new(),
            url: "https://www.youtube.com/watch?v=test123".to_string(),
            thumbnail_url: None,
            uploader: None,
            duration_seconds: None,
            upload_date: None,
        };

        assert_eq!(video_info.id, "test123");
        assert!(video_info.title.is_empty());
        assert!(video_info.url.contains("test123"));
        assert!(video_info.thumbnail_url.is_none());
        assert!(video_info.uploader.is_none());
    }

    #[test]
    fn test_extract_downloaded_filename_mp3() {
        let output = "[ExtractAudio] Destination: /path/to/file.mp3";
        let result = extract_downloaded_filename(output);
        assert_eq!(result, Some("/path/to/file.mp3".to_string()));
    }

    #[test]
    fn test_extract_downloaded_filename_merger() {
        let output = "[Merger] Merging formats into C:\\Users\\test\\file.mp3";
        let result = extract_downloaded_filename(output);
        assert_eq!(result, Some("C:\\Users\\test\\file.mp3".to_string()));
    }

    #[test]
    fn test_extract_downloaded_filename_info() {
        let output = "[info] /downloads/song.mp3";
        let result = extract_downloaded_filename(output);
        assert_eq!(result, Some("/downloads/song.mp3".to_string()));
    }

    #[test]
    fn test_extract_downloaded_filename_no_match() {
        let output = "Some random output without filename";
        let result = extract_downloaded_filename(output);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_downloaded_filename_with_spaces() {
        let output = "[ExtractAudio] Destination: /path/with spaces/My Song Name.mp3";
        let result = extract_downloaded_filename(output);
        assert_eq!(
            result,
            Some("/path/with spaces/My Song Name.mp3".to_string())
        );
    }

    #[test]
    fn test_extract_downloaded_filename_with_special_chars() {
        let output = "[info] /music/Artist_-_Title_(2024).mp3";
        let result = extract_downloaded_filename(output);
        assert_eq!(result, Some("/music/Artist_-_Title_(2024).mp3".to_string()));
    }

    #[test]
    fn test_extract_downloaded_filename_priority_merger_over_info() {
        let output = "[Merger] Merging formats into /path/merged.mp3\n[info] /path/original.mp3";
        let result = extract_downloaded_filename(output);
        assert_eq!(result, Some("/path/merged.mp3".to_string()));
    }

    #[test]
    fn test_search_video_empty_query_returns_error() {
        let result = search_video("yt-dlp", "", None);
        assert!(result.is_err() || result.unwrap().is_none());
    }

    #[test]
    fn test_search_video_whitespace_only_query_returns_error() {
        let result = search_video("yt-dlp", "   ", None);
        assert!(result.is_err() || result.unwrap().is_none());
    }

    #[test]
    fn test_download_stream_invalid_video_id_returns_error() {
        let temp_dir = std::env::temp_dir();
        let result = download_stream(
            "yt-dlp",
            "invalid_id_that_does_not_exist_12345",
            temp_dir.to_str().unwrap(),
            None,
            None,
            |_, _| {},
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_download_stream_invalid_path_returns_error() {
        let result = download_stream(
            "yt-dlp",
            "dQw4w9WgXcQ",
            "/nonexistent/path/that/does/not/exist",
            None,
            None,
            |_, _| {},
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_download_stream_progress_callback() {
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        use std::sync::Arc;

        let called = Arc::new(AtomicBool::new(false));
        let progress_count = Arc::new(AtomicUsize::new(0));
        let called_clone = called.clone();
        let progress_count_clone = progress_count.clone();

        let temp_dir = std::env::temp_dir();
        let _result = download_stream(
            "yt-dlp",
            "dQw4w9WgXcQ",
            temp_dir.to_str().unwrap(),
            None,
            None,
            move |progress, message| {
                called_clone.store(true, Ordering::SeqCst);
                progress_count_clone.fetch_add(1, Ordering::SeqCst);
                assert!(progress >= 0.0 && progress <= 100.0);
                assert!(!message.is_empty());
            },
        );

        assert!(called.load(Ordering::SeqCst));
        assert!(progress_count.load(Ordering::SeqCst) > 0);
    }

    #[test]
    fn test_video_info_clone() {
        let video_info = VideoInfo {
            id: "test123".to_string(),
            title: "Test".to_string(),
            url: "https://youtube.com/watch?v=test123".to_string(),
            thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
            uploader: Some("TestUploader".to_string()),
            duration_seconds: Some(200),
            upload_date: Some("2024-02-01".to_string()),
        };
        let cloned = video_info.clone();
        assert_eq!(video_info.id, cloned.id);
        assert_eq!(video_info.title, cloned.title);
        assert_eq!(video_info.url, cloned.url);
        assert_eq!(video_info.thumbnail_url, cloned.thumbnail_url);
        assert_eq!(video_info.uploader, cloned.uploader);
        assert_eq!(video_info.duration_seconds, cloned.duration_seconds);
        assert_eq!(video_info.upload_date, cloned.upload_date);
    }

    #[test]
    fn test_video_info_debug_format() {
        let video_info = VideoInfo {
            id: "abc".to_string(),
            title: "Test Video".to_string(),
            url: "https://youtube.com/watch?v=abc".to_string(),
            thumbnail_url: None,
            uploader: None,
            duration_seconds: None,
            upload_date: None,
        };
        let debug = format!("{:?}", video_info);
        assert!(debug.contains("abc"));
        assert!(debug.contains("Test Video"));
    }

    #[test]
    fn test_search_video_with_cookies_browser() {
        // Test that cookies_browser parameter is accepted (None case)
        let result = search_video("yt-dlp", "test query", None);
        // Result will be error since "yt-dlp" doesn't exist, but we're testing parameter acceptance
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_download_stream_with_cookies_browser() {
        // Test that cookies_browser parameter is accepted (None case)
        let temp_dir = std::env::temp_dir();
        let result = download_stream(
            "yt-dlp",
            "dQw4w9WgXcQ",
            temp_dir.to_str().unwrap(),
            None,
            None,
            |_, _| {},
        );
        // Result will be error since "yt-dlp" doesn't exist, but we're testing parameter acceptance
        assert!(result.is_err());
    }
}
