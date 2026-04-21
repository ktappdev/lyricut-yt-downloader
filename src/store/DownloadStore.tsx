import { createContext, useContext, useState, ReactNode } from 'react';

export type DownloadMode = 'audio' | 'video' | 'playlist';
export type AudioMode = 'official' | 'raw' | 'clean';
export type CookieBrowser = 'chrome' | 'firefox' | 'brave' | 'edge' | 'none';

export interface CsvTrackMetadata {
  artist_names?: string;
  track_name?: string;
  album_name?: string;
  artist_genres?: string;
  album_release_date?: string;
  bpm_tempo?: string;
}

export interface CsvTrackEntry {
  row_number: number;
  metadata: CsvTrackMetadata;
  search_query: string;
}

export interface CsvImportResult {
  tracks: CsvTrackEntry[];
  total_count: number;
  success_count: number;
  error_count: number;
  errors: string[];
}

interface DownloadState {
  url: string;
  mode: DownloadMode;
  audioMode: AudioMode;
  downloadPath: string;
  inputText: string;
  csvData: CsvImportResult | null;
  progress: number;
  status: string;
  itemCount: number;
  currentItem: number;
  cookiesBrowser: CookieBrowser;
}

interface DownloadStoreContextType extends DownloadState {
  setUrl: (url: string) => void;
  setMode: (mode: DownloadMode) => void;
  setAudioMode: (audioMode: AudioMode) => void;
  setDownloadPath: (path: string) => void;
  setInputText: (text: string) => void;
  setCsvData: (data: CsvImportResult | null) => void;
  setProgress: (progress: number) => void;
  setStatus: (status: string) => void;
  setItemCount: (count: number) => void;
  setCurrentItem: (item: number) => void;
  setCookiesBrowser: (browser: CookieBrowser) => void;
  reset: () => void;
}

const defaultState: DownloadState = {
  url: '',
  mode: 'video',
  audioMode: 'clean',
  downloadPath: '',
  inputText: '',
  csvData: null,
  progress: 0,
  status: '',
  itemCount: 0,
  currentItem: 0,
  cookiesBrowser: 'chrome',
};

const DownloadStoreContext = createContext<DownloadStoreContextType | undefined>(undefined);

export function DownloadProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<DownloadState>(defaultState);

  const setUrl = (url: string) => {
    setState(prev => ({ ...prev, url }));
  };

  const setMode = (mode: DownloadMode) => {
    setState(prev => ({ ...prev, mode }));
  };

  const setAudioMode = (audioMode: AudioMode) => {
    setState(prev => ({ ...prev, audioMode }));
  };

  const setDownloadPath = (downloadPath: string) => {
    setState(prev => ({ ...prev, downloadPath }));
  };

  const setInputText = (inputText: string) => {
    setState(prev => ({ ...prev, inputText }));
  };

  const setCsvData = (csvData: CsvImportResult | null) => {
    setState(prev => ({ ...prev, csvData }));
  };

  const setProgress = (progress: number) => {
    setState(prev => ({ ...prev, progress }));
  };

  const setStatus = (status: string) => {
    setState(prev => ({ ...prev, status }));
  };

  const setItemCount = (itemCount: number) => {
    setState(prev => ({ ...prev, itemCount }));
  };

  const setCurrentItem = (currentItem: number) => {
    setState(prev => ({ ...prev, currentItem }));
  };

  const setCookiesBrowser = (cookiesBrowser: CookieBrowser) => {
    setState(prev => ({ ...prev, cookiesBrowser }));
  };

  const reset = () => {
    setState(defaultState);
  };

  return (
    <DownloadStoreContext.Provider
      value={{
        ...state,
        setUrl,
        setMode,
        setAudioMode,
        setDownloadPath,
        setInputText,
        setCsvData,
        setProgress,
        setStatus,
        setItemCount,
        setCurrentItem,
        setCookiesBrowser,
        reset,
      }}
    >
      {children}
    </DownloadStoreContext.Provider>
  );
}

export function useDownloadStore() {
  const context = useContext(DownloadStoreContext);
  if (context === undefined) {
    throw new Error('useDownloadStore must be used within a DownloadProvider');
  }
  return context;
}
