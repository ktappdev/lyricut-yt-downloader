import { useState, useEffect, useRef } from "react";
import { Button } from "./components/ui/Button";
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from "./components/ui/Card";
import { MainLayout, MainLayoutHeader, MainLayoutFooter, AudioModeSelector, DownloadInput, ActionButtons, ProgressIndicator } from "./components/layout/MainLayout";
import { DownloadProvider, useDownloadStore, AudioMode, CsvImportResult } from "./store/DownloadStore";
import { AboutModal } from "./components/AboutModal";
import { SetupOverlay } from "./components/SetupOverlay";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { FolderOpen, RotateCcw, Settings, Heart, Info, CheckCircle2, XCircle, Download } from "lucide-react";

type SetupStatus = "checking" | "downloading" | "ready" | "error";

function SettingsDropdown({ 
  downloadPath, 
  onChangePath, 
  onOpenFolder,
  ytdlpStatus,
  ytdlpPath,
  onCheckYtdlp,
  onDownloadYtdlp,
  isOpen,
  onClose 
}: { 
  downloadPath: string;
  onChangePath: () => void;
  onOpenFolder: () => void;
  ytdlpStatus: 'detected' | 'missing' | 'checking' | 'downloading';
  ytdlpPath: string;
  onCheckYtdlp: () => void;
  onDownloadYtdlp: () => void;
  isOpen: boolean;
  onClose: () => void;
}) {
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        onClose();
      }
    };
    
    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  const folderName = downloadPath ? downloadPath.split('/').filter(Boolean).pop() || downloadPath : 'Downloads';

  return (
    <div 
      ref={dropdownRef}
      className="absolute right-0 top-full mt-2 w-72 bg-[#1a1a1a] border border-white/10 rounded-lg shadow-xl z-50 animate-fade-in"
    >
      <div className="p-4 border-b border-white/5">
        <h3 className="text-sm font-semibold text-white flex items-center gap-2">
          <Settings className="w-4 h-4 text-red-500" />
          Settings
        </h3>
      </div>
      
      {/* yt-dlp Status Section */}
      <div className="p-4 border-b border-white/5 space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-sm text-gray-400">yt-dlp:</span>
          <div className="flex items-center gap-2">
            {ytdlpStatus === 'detected' && (
              <>
                <CheckCircle2 className="w-4 h-4 text-green-500" />
                <span className="text-sm text-green-500 font-medium">Detected</span>
              </>
            )}
            {ytdlpStatus === 'missing' && (
              <>
                <XCircle className="w-4 h-4 text-red-500" />
                <span className="text-sm text-red-500 font-medium">Missing</span>
              </>
            )}
            {ytdlpStatus === 'checking' && (
              <span className="text-sm text-gray-400 font-medium">Checking...</span>
            )}
            {ytdlpStatus === 'downloading' && (
              <span className="text-sm text-yellow-500 font-medium">Downloading...</span>
            )}
          </div>
        </div>
        {ytdlpPath && ytdlpStatus === 'detected' && (
          <div className="text-xs text-gray-500 break-all">{ytdlpPath}</div>
        )}
        <div className="flex gap-2">
          <Button 
            variant="outline" 
            size="sm" 
            onClick={onCheckYtdlp} 
            className="flex-1"
            disabled={ytdlpStatus === 'checking' || ytdlpStatus === 'downloading'}
          >
            Check
          </Button>
          {ytdlpStatus === 'missing' && (
            <Button 
              variant="ghost" 
              size="sm" 
              onClick={onDownloadYtdlp} 
              className="flex-1 text-red-400 hover:text-red-300"
              disabled={ytdlpStatus === 'downloading'}
            >
              <Download className="w-4 h-4 mr-1" />
              Download
            </Button>
          )}
        </div>
      </div>

      {/* Download Location Section */}
      <div className="p-4 space-y-3">
        <div className="flex items-center gap-2 text-sm font-medium text-white mb-2">
          <FolderOpen className="w-4 h-4 text-red-500" />
          Download Location
        </div>
        <div className="flex items-center justify-between">
          <span className="text-sm text-gray-400">Folder:</span>
          <span className="text-sm text-white font-medium">{folderName}</span>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={onChangePath} className="flex-1">
            Change
          </Button>
          <Button variant="ghost" size="sm" onClick={onOpenFolder} className="flex-1">
            Open
          </Button>
        </div>
      </div>
    </div>
  );
}

function HeaderActions({ 
  audioMode, 
  lineCount, 
  onReset,
  onSettingsClick,
  settingsOpen,
  downloadPath,
  onChangePath,
  onOpenFolder,
  ytdlpStatus,
  ytdlpPath,
  onCheckYtdlp,
  onDownloadYtdlp,
  onCloseSettings
}: { 
  audioMode: AudioMode;
  lineCount: number;
  onReset: () => void;
  onSettingsClick: () => void;
  settingsOpen: boolean;
  downloadPath: string;
  onChangePath: () => void;
  onOpenFolder: () => void;
  ytdlpStatus: 'detected' | 'missing' | 'checking' | 'downloading';
  ytdlpPath: string;
  onCheckYtdlp: () => void;
  onDownloadYtdlp: () => void;
  onCloseSettings: () => void;
}) {
  return (
    <div className="flex items-center gap-4">
      <div className="hidden sm:flex items-center gap-4 text-sm text-gray-400">
        <div className="flex items-center gap-2">
          <span className="text-gray-500">Mode:</span>
          <span className="text-white capitalize">{audioMode}</span>
        </div>
        <div className="w-px h-4 bg-white/10" />
        <div className="flex items-center gap-2">
          <span className="text-gray-500">Items:</span>
          <span className="text-white">{lineCount}</span>
        </div>
      </div>
      <Button onClick={onReset} variant="ghost" size="sm" className="text-gray-400 hover:text-white">
        <RotateCcw className="w-4 h-4 sm:mr-2" />
        <span className="hidden sm:inline">Reset</span>
      </Button>
      <div className="relative">
        <Button 
          onClick={onSettingsClick} 
          variant="outline" 
          size="sm"
          className="text-gray-400 hover:text-white"
        >
          <Settings className="w-4 h-4" />
        </Button>
        <SettingsDropdown
          downloadPath={downloadPath}
          onChangePath={onChangePath}
          onOpenFolder={onOpenFolder}
          ytdlpStatus={ytdlpStatus}
          ytdlpPath={ytdlpPath}
          onCheckYtdlp={onCheckYtdlp}
          onDownloadYtdlp={onDownloadYtdlp}
          isOpen={settingsOpen}
          onClose={onCloseSettings}
        />
      </div>
    </div>
  );
}

function DownloadForm() {
  const {
    downloadPath,
    audioMode,
    inputText,
    csvData,
    progress,
    status,
    currentItem,
    itemCount,
    setAudioMode,
    setInputText,
    setCsvData,
    setDownloadPath,
    setItemCount,
    setCurrentItem,
    setProgress,
    setStatus,
    reset,
  } = useDownloadStore();

  const [settingsOpen, setSettingsOpen] = useState(false);
  const [ytdlpStatus, setYtdlpStatus] = useState<'detected' | 'missing' | 'checking' | 'downloading'>('checking');
  const [ytdlpPath, setYtdlpPath] = useState('');

  useEffect(() => {
    const initializeDownloadPath = async () => {
      if (!downloadPath) {
        try {
          const defaultPath = await downloadDir();
          setDownloadPath(defaultPath);
        } catch (error) {
          console.error("Failed to get default download directory:", error);
        }
      }
    };
    
    initializeDownloadPath();
  }, [downloadPath, setDownloadPath]);

  useEffect(() => {
    checkYtdlpStatus();
  }, []);

  const checkYtdlpStatus = async () => {
    setYtdlpStatus('checking');
    try {
      const path = await invoke<string>("check_ytdlp");
      setYtdlpPath(path);
      setYtdlpStatus('detected');
    } catch (error) {
      console.error("yt-dlp not detected:", error);
      setYtdlpPath('');
      setYtdlpStatus('missing');
    }
  };

  const handleDownloadYtdlp = async () => {
    setYtdlpStatus('downloading');
    try {
      const path = await invoke<string>("download_ytdlp");
      setYtdlpPath(path);
      setYtdlpStatus('detected');
      setStatus('yt-dlp installed successfully');
    } catch (error) {
      console.error("Failed to download yt-dlp:", error);
      setYtdlpStatus('missing');
      setStatus(`Failed to download yt-dlp: ${error}`);
    }
  };

  const handleChangePath = async () => {
    try {
      const result = await invoke<string | null>("set_download_path");
      if (result) {
        setDownloadPath(result);
      }
    } catch (error) {
      console.error("Failed to set download path:", error);
    }
  };

  const handleOpenFolder = async () => {
    if (downloadPath) {
      try {
        await invoke("open_folder", { path: downloadPath });
      } catch (error) {
        console.error("Failed to open folder:", error);
        setStatus(`Failed to open folder: ${error}`);
      }
    } else {
      setStatus("No download path selected");
    }
  };

  const handleImportCsv = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'CSV', extensions: ['csv'] }]
      });

      if (!selected) {
        return;
      }

      const filePath = selected as string;
      const content = await invoke<string>("read_file_command", { path: filePath });
      const result = await invoke<CsvImportResult>("parse_csv_command", { 
        csvContent: content, 
        audioMode: audioMode as string 
      });

      if (result && result.success_count > 0) {
        setCsvData(result);
        const queries = result.tracks.map(t => t.search_query).join('\n');
        setInputText(queries);
        setStatus(`Imported ${result.success_count} tracks from CSV`);
      } else if (result) {
        setStatus(`CSV import had ${result.error_count} errors`);
      }
    } catch (error) {
      console.error("Failed to import CSV:", error);
      setStatus(`CSV import failed: ${error}`);
    }
  };

  const handleDownload = async () => {
    if (!inputText.trim()) {
      console.warn("No input provided");
      return;
    }

    try {
      console.log("Starting download process...");
      const result = await invoke<{
        items: Array<{
          input_type: string;
          original_input: string;
          processed_query: string;
          video_id: string | null;
        }>;
        total_count: number;
        url_count: number;
        search_count: number;
      }>("process_input", { inputText, audioMode: audioMode as string });

      console.log("Processed input result:", result);
      setItemCount(result.total_count);
      setCurrentItem(0);
      setStatus("Starting download...");
      setProgress(0);

      if (!downloadPath) {
        setStatus("Error: No download path set");
        return;
      }

      const outputPath = downloadPath;

      const canUseCsvMetadata = csvData && csvData.tracks.length === result.items.length;
      const errors: string[] = [];

      for (let i = 0; i < result.items.length; i++) {
        const item = result.items[i];
        setCurrentItem(i + 1);
        setProgress((i / result.total_count) * 100);
        
        let metadataOverride = null;
        if (canUseCsvMetadata) {
            const track = csvData.tracks[i];
            if (item.original_input.trim() === track.search_query.trim() || 
                item.processed_query.includes(track.search_query.trim())) {
                metadataOverride = {
                    title: track.metadata.track_name,
                    artist: track.metadata.artist_names,
                    album: track.metadata.album_name,
                    genre: track.metadata.artist_genres,
                    year: track.metadata.album_release_date,
                };
            }
        }

        try {
          let videoId = item.video_id;

          if (item.input_type === "SearchQuery" && !videoId) {
            setStatus(`Searching: ${item.processed_query}`);
            const videoInfo = await invoke<{ id: string } | null>("search_video_command", {
              query: item.processed_query,
            });
            if (videoInfo) {
                videoId = videoInfo.id;
            }
          }

          if (videoId) {
            setStatus(`Processing: ${item.original_input}`);
            await invoke<string>("process_item", {
              videoId,
              outputPath,
              metadataOverride
            });
          } else {
             setStatus(`Not found: ${item.processed_query}`);
          }
        } catch (itemError) {
          const errMsg = `Item ${i + 1} (${item.original_input}): ${itemError}`;
          console.error(errMsg);
          errors.push(errMsg);
          setStatus(`Error: ${errMsg}`);
        }
      }
      
      setProgress(100);
      if (errors.length > 0) {
        setStatus(`Failed ${errors.length}/${result.total_count}: ${errors.join(' | ')}`);
      } else {
        setStatus("Download complete!");
      }
    } catch (error) {
      console.error("Download failed:", error);
      setStatus(`Error: ${error}`);
    }
  };

  const lineCount = inputText.split('\n').filter(line => line.trim()).length;

  return {
    content: (
      <div className="space-y-6 animate-fade-in">
        <Card>
          <CardHeader>
            <CardTitle>Download Queue</CardTitle>
            <CardDescription>
              Paste YouTube URLs or song names to download
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <DownloadInput
              value={inputText}
              onChange={setInputText}
              placeholder="Paste links or song names (one per line)..."
            />

            {lineCount > 0 && (
              <div className="flex items-center gap-2 text-sm text-gray-400">
                <Info className="w-4 h-4" />
                <span>{lineCount} item{lineCount !== 1 ? 's' : ''} in queue</span>
              </div>
            )}

            <div className="flex flex-col gap-3 pt-2">
              <label className="text-sm font-medium text-gray-300">Audio Mode</label>
              <AudioModeSelector
                value={audioMode}
                onChange={(mode: AudioMode) => setAudioMode(mode)}
              />
            </div>

            <ActionButtons
              onImportCsv={handleImportCsv}
              onDownload={handleDownload}
              isImportDisabled={inputText.length > 0}
            />
          </CardContent>
        </Card>

        {(itemCount > 0 || progress > 0) && (
          <Card className="border-red-500/20">
            <CardContent className="pt-6">
              <ProgressIndicator
                progress={progress}
                status={status}
                currentItem={currentItem}
                itemCount={itemCount}
              />
            </CardContent>
          </Card>
        )}
      </div>
    ),
    headerActions: (
      <HeaderActions
        audioMode={audioMode}
        lineCount={lineCount}
        onReset={reset}
        onSettingsClick={() => setSettingsOpen(true)}
        settingsOpen={settingsOpen}
        downloadPath={downloadPath}
        onChangePath={handleChangePath}
        onOpenFolder={handleOpenFolder}
        ytdlpStatus={ytdlpStatus}
        ytdlpPath={ytdlpPath}
        onCheckYtdlp={checkYtdlpStatus}
        onDownloadYtdlp={handleDownloadYtdlp}
        onCloseSettings={() => setSettingsOpen(false)}
      />
    )
  };
}

function App() {
  const [showAbout, setShowAbout] = useState(false);
  const [setupStatus, setSetupStatus] = useState<SetupStatus>("checking");

  useEffect(() => {
    const checkAndDownload = async () => {
      try {
        await invoke<string>("check_ytdlp");
        setSetupStatus("ready");
      } catch {
        setSetupStatus("downloading");
        try {
          await invoke<string>("download_ytdlp");
        } catch (error) {
          console.error("Failed to download yt-dlp:", error);
          setSetupStatus("error");
        }
      }
    };

    checkAndDownload();
  }, []);

  const handleRetry = async () => {
    setSetupStatus("downloading");
    try {
      await invoke<string>("download_ytdlp");
    } catch (error) {
      console.error("Failed to download yt-dlp:", error);
      setSetupStatus("error");
    }
  };

  const handleSetupComplete = () => {
    setSetupStatus("ready");
  };

  if (setupStatus !== "ready") {
    return (
      <SetupOverlay 
        onComplete={handleSetupComplete} 
        onRetry={handleRetry} 
      />
    );
  }

  return (
    <DownloadProvider>
      <AppContent showAbout={showAbout} setShowAbout={setShowAbout} />
    </DownloadProvider>
  );
}

function AppContent({ showAbout, setShowAbout }: { showAbout: boolean; setShowAbout: (v: boolean) => void }) {
  const form = DownloadForm();
  
  return (
    <>
      <MainLayout
        header={
          <MainLayoutHeader
            title="Lyricut YT Downloader"
            description="Download YouTube audio with metadata tagging"
            actions={form.headerActions}
          />
        }
        footer={
          <MainLayoutFooter>
            <div className="flex items-center justify-center gap-2 w-full">
              <span className="text-gray-500">Built with</span>
              <Heart className="w-4 h-4 text-red-500 fill-red-500" />
              <span className="text-gray-500">by</span>
              <button 
                onClick={() => setShowAbout(true)}
                className="text-white hover:text-red-400 transition-colors font-medium"
              >
                Ken Taylor
              </button>
            </div>
          </MainLayoutFooter>
        }
      >
        {form.content}
      </MainLayout>
      
      <AboutModal isOpen={showAbout} onClose={() => setShowAbout(false)} />
    </>
  );
}

export default App;
