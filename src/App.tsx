import { Button } from "./components/ui/Button";
import { Card, CardHeader, CardTitle, CardContent } from "./components/ui/Card";
import { MainLayout, MainLayoutHeader, MainLayoutFooter, DownloadSettings, AudioModeSelector, DownloadInput, ActionButtons, ProgressIndicator } from "./components/layout/MainLayout";
import { DownloadProvider, useDownloadStore, AudioMode } from "./store/DownloadStore";
import { invoke } from "@tauri-apps/api/core";

function DownloadForm() {
  const {
    downloadPath,
    audioMode,
    inputText,
    progress,
    status,
    currentItem,
    itemCount,
    setAudioMode,
    setInputText,
    setDownloadPath,
    reset,
  } = useDownloadStore();

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
      }
    }
  };

  const handleImportCsv = () => {
    console.log("Import CSV clicked");
  };

  const handleDownload = () => {
    console.log("Download clicked");
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Download Settings</CardTitle>
        </CardHeader>
        <CardContent>
          <DownloadSettings
            downloadPath={downloadPath}
            onChangePath={handleChangePath}
            onOpenFolder={handleOpenFolder}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Input</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <DownloadInput
            value={inputText}
            onChange={setInputText}
            placeholder="Paste links or song names (one per line)..."
          />

          <div className="flex flex-col gap-2">
            <label className="text-sm font-medium">Search Mode</label>
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
        <Card>
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

      <div className="text-sm text-muted-foreground">
        <p>Current settings:</p>
        <p>Path: {downloadPath || '(default)'}</p>
        <p>Audio Mode: {audioMode}</p>
        <p>Lines: {inputText.split('\n').filter(line => line.trim()).length}</p>
        <Button onClick={reset} variant="outline" size="sm" className="mt-2">
          Reset All
        </Button>
      </div>
    </div>
  );
}

function App() {
  return (
    <DownloadProvider>
      <MainLayout
        header={
          <MainLayoutHeader
            title="YouTube Downloader"
            description="Download YouTube videos, audio, or entire playlists"
          />
        }
        footer={<MainLayoutFooter>YouTube Downloader - Tauri App</MainLayoutFooter>}
      >
        <DownloadForm />
      </MainLayout>
    </DownloadProvider>
  );
}

export default App;
