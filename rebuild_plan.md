# Project Rebuild Plan: Modern YouTube Downloader (Tauri)

## 1. Project Overview
**Goal:** Rebuild a legacy Python/PyQt YouTube Downloader application using **Tauri (Rust)** and a modern frontend framework.
**Purpose:** The application allows users to download music from YouTube via direct links, search queries, or bulk lists (CSV). It handles audio conversion (MP3), filename cleaning, and metadata tagging automatically.

## 2. Architecture & Tech Stack
*   **Core Framework:** [Tauri](https://tauri.app/) (v2 recommended).
*   **Backend (Rust):** Handles heavy lifting—YouTube interaction, file system operations, audio conversion, and multithreading.
*   **Frontend (Web Tech):** React, Vue, Svelte, or SolidJS (User's choice). Should provide a modern, responsive UI.
*   **External Tools/Libraries (Concepts):**
    *   **YouTube Scraper:** Library to search and extract video streams (e.g., `rustube`, `youtube_dl` bindings, or a custom scraper).
    *   **Audio Conversion:** FFmpeg (bundled or system-dependent) to convert streams (WebM/MP4) to MP3.
    *   **Metadata:** Library to write ID3 tags (Artist, Title, Album, Cover Art) to MP3 files.
    *   **CSV Parser:** For reading bulk import files.

## 3. Functional Requirements (Features)

### 3.1. Input Methods
The application must support a versatile text input area that handles two types of content automatically:
1.  **Direct URLs:**
    *   Detect if the input text contains valid YouTube URLs (single or multiple).
    *   **Action:** Download the specific videos directly.
2.  **Search Queries (Song Names):**
    *   If no URLs are detected, treat each line of text as a song title/artist.
    *   **Action:** Perform a YouTube search for each line and download the **first result**.

### 3.2. Search Modifiers (Audio Type)
Users can select a "Search Mode" that appends keywords to the search query to find specific versions of a song:
*   **Official Audio:** Appends "official audio" to the query.
*   **Raw Audio:** Appends "raw audio".
*   **Clean Audio:** Appends "clean audio" (useful for radio edits).

### 3.3. Bulk Import (Spotify/CSV)
*   **Feature:** Button to import a CSV file (e.g., exported from Spotify or a playlist manager).
*   **Parsing Logic:**
    *   Read specific headers: `Artist Name(s)`, `Track Name`, `Album Name`, `Artist Genres`, `Album Release Date`, `BPM/Tempo`.
    *   **Search Query:** Construct query as `"{Artist Name} - {Track Name} {Selected Audio Type}"`.
*   **Metadata Injection:** Use the data from the CSV row (Artist, Album, Genre, Year) to populate the ID3 tags of the final MP3 file.

### 3.4. Audio Processing Pipeline (The "Secret Sauce")
Every download must go through this strict pipeline:
1.  **Download:** Fetch the highest quality audio-only stream (usually WebM or MP4).
2.  **Filename Cleaning:** Before saving/converting, remove "garbage" text from the filename/title to make it look like a library track.
    *   *Strings to remove:* `(Official Video)`, `[Audio HD]`, `(Lyrics)`, `(Official Music Video)`, `[Clean]`, etc.
3.  **Conversion:** Convert the downloaded container to **MP3**.
4.  **Tagging:** Apply ID3 tags (Title, Artist) derived from the video title or CSV data.
5.  **Cleanup:** Delete the original temporary video file, keeping only the MP3.

### 3.5. File Management
*   **Default Path:** Defaults to `~/Downloads/Youtube/Multi`.
*   **Custom Path:** User can select a different download folder via a system dialog.
*   **Open Folder:** Button to open the current download directory in the OS file explorer (Finder/Explorer).

## 4. User Interface (UI) Specification
The UI should be clean, modern, and single-window (suggested size: ~800x600).

**Layout Components:**
1.  **Header/Settings Area:**
    *   **Download Path Input:** Read-only field showing current path.
    *   **Change Path Button:** Opens folder picker.
    *   **Open Folder Button:** Opens directory.
2.  **Main Input Area:**
    *   **Multi-line Text Area:** Large box for pasting links or song lists.
    *   **Placeholder Text:** "Paste links or song names (one per line)..."
3.  **Control Panel:**
    *   **Radio Buttons (Search Mode):** [Official Audio] | [Raw Audio] | [Clean Audio].
    *   **Import Button:** "Import CSV" (disables if text area has text).
4.  **Action Area:**
    *   **Download Button:** Large, prominent button to start the process.
    *   **Progress Bar:** Visual indicator of batch progress (0-100%).
    *   **Status Label:** Shows current action (e.g., "Searching for 'Song X'...", "Converting...").
    *   **Counter:** "Downloading 3 of 10".

## 5. Implementation Roadmap (To-Do List)

### Phase 1: Project Setup & UI
- [x] Initialize new Tauri project (`npm create tauri-app`).
- [x] Set up Frontend (React/Vue/etc.) with a component library (e.g., Tailwind, Shadcn, or Material UI).
- [x] Build the **Main Layout**:
    - [ ] Header with settings.
    - [ ] Text Area for input.
    - [ ] Radio buttons for Audio Mode.
    - [ ] Action buttons (Download, Import).
- [x] Implement State Management (store user input, selected mode, download path).

### Phase 2: Backend Logic (Rust)
- [x] **Command:** `set_download_path` & `open_folder` (using Tauri APIs).
- [ ] **Command:** `process_input` (Main entry point).
    - [ ] Implement Regex to distinguish URL vs. Text.
- [ ] **Module:** `youtube_client`.
    - [ ] Integrate a Rust YouTube library (e.g., `rustube` or execute `yt-dlp` as a sidecar binary).
    - [ ] Implement `search_video(query)` -> returns Video ID.
    - [ ] Implement `download_stream(video_id, path)`.
- [ ] **Module:** `file_processor`.
    - [ ] Implement `clean_filename(original_name)` logic (port the list of banned strings).
    - [ ] Implement `convert_to_mp3(input_path, output_path)` (via FFmpeg sidecar or library).

### Phase 3: Advanced Features
- [ ] **CSV Import Logic:**
    - [ ] Create a Rust command to parse CSV files.
    - [ ] Extract metadata fields (Artist, Album, etc.).
    - [ ] Pass this data to the download pipeline.
- [ ] **Metadata Tagging:**
    - [ ] Integrate a crate like `id3` or `lofty` to write tags to the MP3s.
    - [ ] Logic: Use CSV data if available; otherwise, infer from Video Title.

### Phase 4: Threading & Feedback
- [ ] Ensure downloads run asynchronously (don't freeze UI).
- [ ] Emit Tauri Events to Frontend:
    - [ ] `progress_update`: { value: 50, label: "Downloading..." }
    - [ ] `item_complete`: { success: true, filename: "Song.mp3" }
    - [ ] `batch_complete`.

### Phase 5: Distribution
- [ ] Configure `tauri.conf.json` allowlists (fs, shell, dialog).
- [ ] Bundle FFmpeg/yt-dlp binaries if using sidecar approach.
- [ ] Build & Test on target OS (macOS/Windows).

## 6. Detailed Logic Reference (Pseudo-Code)

**Filename Cleaning List (Port this list):**
> Remove: `[Audio HD]`, `(Radio Mix)`, `(Official Video)`, `(lyrics)`, `(Radio Edit)`, `[High Quality]`, `(Official Music Video)`, `(Audio)`, `[Clean version]`, `[visualizer]`, etc.

**Search Logic:**
```rust
fn construct_query(line: String, mode: AudioMode) -> String {
    let suffix = match mode {
        AudioMode::Official => "official audio",
        AudioMode::Raw => "raw audio",
        AudioMode::Clean => "clean audio",
    };
    format!("{} {}", line, suffix)
}
```

**CSV Headers to Expect:**
*   `Artist Name(s)`
*   `Track Name`
*   `Album Name`
*   `Artist Genres`
*   `Album Release Date`
