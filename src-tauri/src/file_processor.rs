use regex::Regex;
use std::path::Path;
use std::process::{Command, Stdio};

const BANNED_STRINGS: &[&str] = &[
    "[Audio HD]",
    "(Radio Mix)",
    "(Official Video)",
    "(lyrics)",
    "(Radio Edit)",
    "[High Quality]",
    "(Official Music Video)",
    "(Audio)",
    "[Clean version]",
    "[visualizer]",
    "[Official]",
    "[Lyric Video]",
    "[Lyrics]",
    "(Lyric Video)",
    "(Explicit)",
    "[Explicit]",
    "(Clean)",
    "[Live]",
    "(Studio)",
    "[Studio]",
    "[Remastered]",
    "[Remix]",
    "(Remix)",
    "[DJ Mix]",
    "(DJ Mix)",
    "[Acoustic]",
    "(Acoustic)",
    "[Instrumental]",
    "(Instrumental)",
    "[Extended]",
    "(Extended)",
    "[Edit]",
    "(Edit)",
    "[Version]",
    "(Version)",
    "[Mixed]",
    "(Mixed)",
];

pub fn clean_filename(original_name: &str) -> String {
    let mut result = original_name.to_string();

    for banned in BANNED_STRINGS {
        let pattern = format!("(?i)\\s*{}", regex::escape(banned));
        if let Ok(regex) = Regex::new(&pattern) {
            result = regex.replace_all(&result, "").to_string();
        }
    }

    let patterns = [r"\s+", r"^\s+|\s+$"];

    for pattern in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            result = regex.replace_all(&result, " ").to_string();
        }
    }

    result.trim().to_string()
}

pub fn convert_to_mp3(input_path: &str, output_path: &str) -> Result<String, String> {
    let input = Path::new(input_path);
    let output = Path::new(output_path);

    if !input.exists() {
        return Err(format!("Input file does not exist: {}", input_path));
    }

    let output_dir = output.parent().ok_or("Invalid output path")?;
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    let output = Command::new("ffmpeg")
        .args([
            "-i",
            input_path,
            "-vn",
            "-acodec",
            "libmp3lame",
            "-q:a",
            "0",
            "-y",
            output_path,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn ffmpeg: {}", e))?;

    let output = output
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for ffmpeg: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffmpeg conversion failed: {}", stderr));
    }

    Ok(output_path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_filename_removes_audio_hd() {
        let result = clean_filename("Song Name [Audio HD]");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_radio_mix() {
        let result = clean_filename("Song Name (Radio Mix)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_official_video() {
        let result = clean_filename("Song Name (Official Video)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_lyrics() {
        let result = clean_filename("Song Name (lyrics)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_radio_edit() {
        let result = clean_filename("Song Name (Radio Edit)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_high_quality() {
        let result = clean_filename("Song Name [High Quality]");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_official_music_video() {
        let result = clean_filename("Song Name (Official Music Video)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_audio() {
        let result = clean_filename("Song Name (Audio)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_clean_version() {
        let result = clean_filename("Song Name [Clean version]");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_visualizer() {
        let result = clean_filename("Song Name [visualizer]");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_multiple_patterns() {
        let result = clean_filename("Song Name [Audio HD] (Official Video)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_handles_case_insensitive() {
        let result = clean_filename("Song Name (LYRICS)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_handles_lowercase() {
        let result = clean_filename("Song Name (lyrics)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_trims_whitespace() {
        let result = clean_filename("   Song Name   ");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_preserves_actual_text() {
        let result = clean_filename("Song Name with Meaningful Text");
        assert_eq!(result, "Song Name with Meaningful Text");
    }

    #[test]
    fn test_clean_filename_handles_empty_string() {
        let result = clean_filename("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_filename_removes_live() {
        let result = clean_filename("Song Name [Live]");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_studio() {
        let result = clean_filename("Song Name (Studio)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_remastered() {
        let result = clean_filename("Song Name [Remastered]");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_remix() {
        let result = clean_filename("Song Name (Remix)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_acoustic() {
        let result = clean_filename("Song Name [Acoustic]");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_extended() {
        let result = clean_filename("Song Name (Extended)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_complex_example() {
        let result = clean_filename("Artist Name - Song Title (Official Video) [Audio HD]");
        assert_eq!(result, "Artist Name - Song Title");
    }

    #[test]
    fn test_clean_filename_removes_explicit() {
        let result = clean_filename("Song Name (Explicit)");
        assert_eq!(result, "Song Name");
    }

    #[test]
    fn test_clean_filename_removes_instrumental() {
        let result = clean_filename("Song Name [Instrumental]");
        assert_eq!(result, "Song Name");
    }
}
