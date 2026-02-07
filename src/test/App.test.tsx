import { render, screen, fireEvent, act } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import App from "../App";
import "@testing-library/jest-dom";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("App Component", () => {
  it("renders YouTube Downloader title", () => {
    render(<App />);
    expect(screen.getByText("YouTube Downloader")).toBeInTheDocument();
  });

  it("renders download settings section", () => {
    render(<App />);
    expect(screen.getByText("Download Settings")).toBeInTheDocument();
    expect(screen.getByPlaceholderText("Select download path...")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Change Path" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Open Folder" })).toBeInTheDocument();
  });

  it("renders input section", () => {
    render(<App />);
    expect(screen.getByText("Input")).toBeInTheDocument();
    expect(screen.getByPlaceholderText("Paste links or song names (one per line)...")).toBeInTheDocument();
  });

  it("renders audio mode selector", () => {
    render(<App />);
    expect(screen.getByRole("button", { name: "Official Audio" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Raw Audio" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Clean Audio" })).toBeInTheDocument();
  });

  it("renders action buttons", () => {
    render(<App />);
    expect(screen.getByRole("button", { name: "Import CSV" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Download" })).toBeInTheDocument();
  });

  it("renders reset all button", () => {
    render(<App />);
    expect(screen.getByRole("button", { name: "Reset All" })).toBeInTheDocument();
  });

  it("updates input text on change", () => {
    render(<App />);
    const textarea = screen.getByPlaceholderText("Paste links or song names (one per line)...");
    fireEvent.change(textarea, { target: { value: "https://youtube.com/watch?v=test\nAnother song" } });
    expect(textarea).toHaveValue("https://youtube.com/watch?v=test\nAnother song");
  });
});

import { invoke } from "@tauri-apps/api/core";

describe("Tauri Commands", () => {
  it("calls set_download_path command when Change Path is clicked", async () => {
    vi.mocked(invoke).mockResolvedValue("/selected/path");

    render(<App />);
    const changePathButton = screen.getByRole("button", { name: "Change Path" });
    await act(async () => {
      fireEvent.click(changePathButton);
    });

    expect(invoke).toHaveBeenCalledWith("set_download_path");
  });

  it("does not call open_folder when path is empty", async () => {
    vi.mocked(invoke).mockResolvedValue(undefined);

    render(<App />);
    const openFolderButton = screen.getByRole("button", { name: "Open Folder" });
    await act(async () => {
      fireEvent.click(openFolderButton);
    });

    expect(invoke).not.toHaveBeenCalledWith("open_folder", { path: "" });
  });
});
