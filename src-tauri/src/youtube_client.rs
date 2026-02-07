use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub url: String,
}

pub fn search_video(query: &str) -> Result<Option<VideoInfo>, String> {
    let search_query = format!("ytsearch10:{}", query);

    let output = Command::new("yt-dlp")
        .args([
            "--dump-json",
            "--no-download",
            "--quiet",
            "--no-warnings",
            &search_query,
        ])
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

            return Ok(Some(VideoInfo { id, title, url }));
        }
    }

    Ok(None)
}

pub fn download_stream(
    video_id: &str,
    output_path: &str,
    on_progress: impl Fn(f64, &str) + Send + 'static,
) -> Result<String, String> {
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);
    let output_template = format!("{}/%(title)s [%(id)s].%(ext)s", output_path);

    on_progress(0.0, "Starting download...");

    let output = Command::new("yt-dlp")
        .args([
            "--format",
            "bestaudio[ext=m4a]/bestaudio[ext=webm]/bestaudio",
            "--output",
            &output_template,
            "--extract-audio",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0",
            "--no-playlist",
            "--no-warnings",
            "--progress",
            &video_url,
        ])
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
}
