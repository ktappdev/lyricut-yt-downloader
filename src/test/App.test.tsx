import { render, screen, fireEvent, act, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import App from "../App";
import "@testing-library/jest-dom";

const invoke = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => invoke(...args),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

describe("App Component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    invoke.mockResolvedValue("yt-dlp found");
  });

  it("renders Lyricut YT Downloader title", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByText("Lyricut YT Downloader")).toBeInTheDocument();
    });
  });

  it("renders input section", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByText("Download Queue")).toBeInTheDocument();
      expect(screen.getByPlaceholderText("Paste links or song names (one per line)...")).toBeInTheDocument();
    });
  });

  it("renders audio mode selector", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByRole("button", { name: /Official/ })).toBeInTheDocument();
      expect(screen.getByRole("button", { name: /Raw/ })).toBeInTheDocument();
      expect(screen.getByRole("button", { name: /Clean/ })).toBeInTheDocument();
    });
  });

  it("renders action buttons", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Import CSV" })).toBeInTheDocument();
      expect(screen.getByRole("button", { name: "Download" })).toBeInTheDocument();
    });
  });

  it("renders reset button in header", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByRole("button", { name: /Reset/ })).toBeInTheDocument();
    });
  });

  it("renders settings button in header", async () => {
    render(<App />);
    await waitFor(() => {
      const settingsButtons = screen.getAllByRole("button").filter(btn => btn.querySelector('svg.lucide-settings'));
      expect(settingsButtons.length).toBeGreaterThan(0);
    });
  });

  it("shows download location dropdown when settings is clicked", async () => {
    render(<App />);
    
    await waitFor(() => {
      expect(screen.getByText("Lyricut YT Downloader")).toBeInTheDocument();
    });
    
    const settingsButton = screen.getAllByRole("button").find(btn => 
      btn.querySelector('svg.lucide-settings')
    );
    
    await act(async () => {
      fireEvent.click(settingsButton!);
    });
    
    await waitFor(() => {
      expect(screen.getByText("Download Location")).toBeInTheDocument();
    });
  });

  it("shows change path button when settings dropdown is open", async () => {
    render(<App />);
    
    await waitFor(() => {
      expect(screen.getByText("Lyricut YT Downloader")).toBeInTheDocument();
    });
    
    const settingsButton = screen.getAllByRole("button").find(btn => 
      btn.querySelector('svg.lucide-settings')
    );
    
    await act(async () => {
      fireEvent.click(settingsButton!);
    });
    
    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Change" })).toBeInTheDocument();
    });
  });

  it("updates input text on change", async () => {
    render(<App />);
    
    await waitFor(() => {
      expect(screen.getByPlaceholderText("Paste links or song names (one per line)...")).toBeInTheDocument();
    });
    
    const textarea = screen.getByPlaceholderText("Paste links or song names (one per line)...");
    fireEvent.change(textarea, { target: { value: "https://youtube.com/watch?v=test\nAnother song" } });
    expect(textarea).toHaveValue("https://youtube.com/watch?v=test\nAnother song");
  });
});

describe("Tauri Commands", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    invoke.mockResolvedValue("yt-dlp found");
  });

  it("calls set_download_path command when Change Path is clicked in settings dropdown", async () => {
    invoke.mockResolvedValueOnce("yt-dlp found").mockResolvedValueOnce("/selected/path");

    render(<App />);
    
    await waitFor(() => {
      expect(screen.getByText("Lyricut YT Downloader")).toBeInTheDocument();
    });
    
    const settingsButton = screen.getAllByRole("button").find(btn => 
      btn.querySelector('svg.lucide-settings')
    );
    
    await act(async () => {
      fireEvent.click(settingsButton!);
    });
    
    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Change" })).toBeInTheDocument();
    });
    
    const changePathButton = screen.getByRole("button", { name: "Change" });
    await act(async () => {
      fireEvent.click(changePathButton);
    });

    expect(invoke).toHaveBeenCalledWith("set_download_path");
  });

  it("does not call open_folder when path is empty", async () => {
    invoke.mockResolvedValueOnce("yt-dlp found");

    render(<App />);
    
    await waitFor(() => {
      expect(screen.getByText("Lyricut YT Downloader")).toBeInTheDocument();
    });
    
    const settingsButton = screen.getAllByRole("button").find(btn => 
      btn.querySelector('svg.lucide-settings')
    );
    
    await act(async () => {
      fireEvent.click(settingsButton!);
    });
    
    await waitFor(() => {
      expect(screen.getByText("Download Location")).toBeInTheDocument();
    });

    expect(invoke).not.toHaveBeenCalledWith("open_folder", { path: "" });
  });
});
