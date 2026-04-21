import * as React from "react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/ui/Input";
import { Textarea } from "@/components/ui/Textarea";
import { FolderOpen, Download, Upload, Play, Radio, Music, Mic2 } from "lucide-react";

interface MainLayoutProps extends React.HTMLAttributes<HTMLDivElement> {
  children?: React.ReactNode;
  header?: React.ReactNode;
  footer?: React.ReactNode;
}

const MainLayout = React.forwardRef<HTMLDivElement, MainLayoutProps>(
  ({ className, children, header, footer, ...props }, ref) => {
    return (
      <div
        ref={ref}
        className={cn(
          "min-h-screen flex flex-col bg-background",
          className
        )}
        {...props}
      >
        {header && (
          <header className="sticky top-0 z-50 bg-gradient-to-b from-[#181818] to-[#121212] border-b border-white/5 shadow-lg shadow-black/20">
            {header}
          </header>
        )}
        <main className="flex-1 container mx-auto p-4 md:p-5 max-w-4xl">
          {children}
        </main>
        {footer && (
          <footer className="border-t border-white/5 bg-[#0f0f0f] mt-auto">
            {footer}
          </footer>
        )}
      </div>
    );
  }
);
MainLayout.displayName = "MainLayout";

interface MainLayoutHeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  title?: string;
  description?: string;
  actions?: React.ReactNode;
}

const MainLayoutHeader = React.forwardRef<HTMLDivElement, MainLayoutHeaderProps>(
  ({ className, title, description, actions, children, ...props }, ref) => {
    return (
      <div
        ref={ref}
        className={cn(
          "flex flex-col md:flex-row md:items-center md:justify-between gap-4 p-4 md:px-6 lg:px-8",
          className
        )}
        {...props}
      >
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-gradient-to-br from-red-500 to-red-600 shadow-lg shadow-red-500/30 overflow-hidden">
            <img src="/logo.png" alt="Logo" className="w-full h-full object-contain" />
          </div>
          <div className="flex-1">
            {children || (
              <>
                {title && (
                  <h1 className="text-xl font-bold tracking-tight text-white">
                    {title}
                  </h1>
                )}
                {description && (
                  <p className="text-gray-400 text-sm mt-0.5">
                    {description}
                  </p>
                )}
              </>
            )}
          </div>
        </div>
        {actions && (
          <div className="flex items-center gap-2">
            {actions}
          </div>
        )}
      </div>
    );
  }
);
MainLayoutHeader.displayName = "MainLayoutHeader";

interface MainLayoutFooterProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
}

const MainLayoutFooter = React.forwardRef<HTMLDivElement, MainLayoutFooterProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <div
        ref={ref}
        className={cn(
          "flex flex-col md:flex-row md:items-center md:justify-center gap-4 p-4 md:px-6 lg:px-8 text-sm text-gray-500",
          className
        )}
        {...props}
      >
        {children}
      </div>
    );
  }
);
MainLayoutFooter.displayName = "MainLayoutFooter";

export type AudioMode = 'official' | 'raw' | 'clean';

interface DownloadSettingsProps {
  downloadPath: string;
  onChangePath: () => void;
  onOpenFolder: () => void;
}

function DownloadSettings({ downloadPath, onChangePath, onOpenFolder }: DownloadSettingsProps) {
  return (
    <div className="flex flex-col sm:flex-row gap-3">
      <div className="flex-1 relative">
        <Input
          value={downloadPath || "~/Downloads/Youtube/Multi"}
          readOnly
          placeholder="Select download path..."
          className="pr-10 bg-[#1a1a1a] border-white/10 text-gray-300"
        />
        <FolderOpen className="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-500" />
      </div>
      <Button variant="outline" size="default" onClick={onChangePath} className="shrink-0">
        Change Path
      </Button>
      <Button variant="ghost" size="default" onClick={onOpenFolder} className="shrink-0">
        <Play className="w-4 h-4 mr-2" />
        Open
      </Button>
    </div>
  );
}

interface AudioModeSelectorProps {
  value: AudioMode;
  onChange: (mode: AudioMode) => void;
  disabled?: boolean;
}

function AudioModeSelector({ value, onChange, disabled }: AudioModeSelectorProps) {
  const modes: { value: AudioMode; label: string; icon: React.ReactNode }[] = [
    { value: 'official', label: 'Official', icon: <Music className="w-4 h-4" /> },
    { value: 'raw', label: 'Raw', icon: <Mic2 className="w-4 h-4" /> },
    { value: 'clean', label: 'Clean', icon: <Radio className="w-4 h-4" /> },
  ];

  return (
    <div className="flex flex-wrap gap-2">
      {modes.map((mode) => (
        <Button
          key={mode.value}
          variant={value === mode.value ? 'default' : 'outline'}
          size="sm"
          onClick={() => onChange(mode.value)}
          disabled={disabled}
          className={cn(
            "flex-1 min-w-[100px] transition-all duration-200",
            value === mode.value 
              ? "shadow-lg shadow-red-500/30" 
              : "hover:border-white/30"
          )}
        >
          {mode.icon}
          <span className="ml-2">{mode.label}</span>
        </Button>
      ))}
    </div>
  );
}

interface DownloadInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
}

function DownloadInput({ value, onChange, placeholder, disabled, className }: DownloadInputProps) {
  return (
    <Textarea
      value={value}
      onChange={(e) => onChange(e.currentTarget.value)}
      placeholder={placeholder || "Paste links or song names (one per line)..."}
      className={cn("min-h-[140px] font-mono text-sm bg-[#1a1a1a]", className)}
      disabled={disabled}
    />
  );
}

interface ActionButtonsProps {
  onImportCsv: () => void;
  onDownload: () => void;
  isImportDisabled?: boolean;
  isDownloadDisabled?: boolean;
}

function ActionButtons({ onImportCsv, onDownload, isImportDisabled, isDownloadDisabled }: ActionButtonsProps) {
  return (
    <div className="flex gap-3 pt-2">
      <Button
        variant="outline"
        onClick={onImportCsv}
        disabled={isImportDisabled}
        className="flex-1"
      >
        <Upload className="w-4 h-4 mr-2" />
        Import CSV
      </Button>
      <Button
        variant="default"
        size="lg"
        onClick={onDownload}
        disabled={isDownloadDisabled}
        className="flex-1 font-semibold"
      >
        <Download className="w-4 h-4 mr-2" />
        Download
      </Button>
    </div>
  );
}

interface ProgressIndicatorProps {
  progress: number;
  status: string;
  currentItem: number;
  itemCount: number;
}

function ProgressIndicator({ progress, status, currentItem, itemCount }: ProgressIndicatorProps) {
  return (
    <div className="space-y-3 animate-fade-in">
      {itemCount > 0 && (
        <div className="flex items-center justify-between text-sm">
          <span className="text-gray-400 truncate flex-1 mr-4">{status}</span>
          <span className="text-white font-medium shrink-0 bg-white/10 px-2 py-0.5 rounded">
            {currentItem} / {itemCount}
          </span>
        </div>
      )}
      <div
        role="progressbar"
        aria-valuenow={progress}
        aria-valuemin={0}
        aria-valuemax={100}
        className="w-full bg-white/10 rounded-full h-2 overflow-hidden"
      >
        <div
          className="h-full rounded-full bg-gradient-to-r from-red-600 to-red-400 transition-all duration-500 ease-out relative overflow-hidden"
          style={{ width: `${progress}%` }}
        >
          {/* Shimmer effect */}
          <div className="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent animate-shimmer" />
        </div>
      </div>
      <div className="text-center text-xs text-gray-500">
        {Math.round(progress)}% complete
      </div>
    </div>
  );
}

export {
  MainLayout,
  MainLayoutHeader,
  MainLayoutFooter,
  DownloadSettings,
  AudioModeSelector,
  DownloadInput,
  ActionButtons,
  ProgressIndicator,
};
