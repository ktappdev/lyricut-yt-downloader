import { render, screen, fireEvent, cleanup } from "@testing-library/react";
import { describe, it, expect, afterEach } from "vitest";
import { DownloadProvider, useDownloadStore } from "../store/DownloadStore";
import "@testing-library/jest-dom";

function TestComponent() {
  const {
    url,
    mode,
    audioMode,
    downloadPath,
    inputText,
    progress,
    status,
    itemCount,
    currentItem,
    setUrl,
    setMode,
    setAudioMode,
    setDownloadPath,
    setInputText,
    setProgress,
    setStatus,
    setItemCount,
    setCurrentItem,
    reset
  } = useDownloadStore();

  return (
    <div>
      <div data-testid="url">{url}</div>
      <div data-testid="mode">{mode}</div>
      <div data-testid="audioMode">{audioMode}</div>
      <div data-testid="downloadPath">{downloadPath}</div>
      <div data-testid="inputText">{inputText}</div>
      <div data-testid="progress">{progress}</div>
      <div data-testid="status">{status}</div>
      <div data-testid="itemCount">{itemCount}</div>
      <div data-testid="currentItem">{currentItem}</div>
      <button onClick={() => setUrl("https://youtube.com/watch?v=abc")}>Set URL</button>
      <button onClick={() => setMode("audio")}>Set Audio Mode</button>
      <button onClick={() => setMode("video")}>Set Video Mode</button>
      <button onClick={() => setMode("playlist")}>Set Playlist Mode</button>
      <button onClick={() => setAudioMode("official")}>Set Official Audio</button>
      <button onClick={() => setAudioMode("raw")}>Set Raw Audio</button>
      <button onClick={() => setAudioMode("clean")}>Set Clean Audio</button>
      <button onClick={() => setDownloadPath("/downloads")}>Set Path</button>
      <button onClick={() => setInputText("song1\nsong2")}>Set Input Text</button>
      <button onClick={() => setProgress(50)}>Set Progress</button>
      <button onClick={() => setStatus("Downloading...")}>Set Status</button>
      <button onClick={() => setItemCount(10)}>Set Item Count</button>
      <button onClick={() => setCurrentItem(3)}>Set Current Item</button>
      <button onClick={reset}>Reset</button>
    </div>
  );
}

describe("DownloadStore", () => {
  afterEach(() => {
    cleanup();
  });

  it("provides default state", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    expect(screen.getByTestId("url")).toHaveTextContent("");
    expect(screen.getByTestId("mode")).toHaveTextContent("video");
    expect(screen.getByTestId("audioMode")).toHaveTextContent("clean");
    expect(screen.getByTestId("downloadPath")).toHaveTextContent("");
    expect(screen.getByTestId("inputText")).toHaveTextContent("");
    expect(screen.getByTestId("progress")).toHaveTextContent("0");
    expect(screen.getByTestId("status")).toHaveTextContent("");
    expect(screen.getByTestId("itemCount")).toHaveTextContent("0");
    expect(screen.getByTestId("currentItem")).toHaveTextContent("0");
  });

  it("sets URL", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set URL"));
    expect(screen.getByTestId("url")).toHaveTextContent("https://youtube.com/watch?v=abc");
  });

  it("sets mode to audio", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Audio Mode"));
    expect(screen.getByTestId("mode")).toHaveTextContent("audio");
  });

  it("sets mode to video", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Video Mode"));
    expect(screen.getByTestId("mode")).toHaveTextContent("video");
  });

  it("sets mode to playlist", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Playlist Mode"));
    expect(screen.getByTestId("mode")).toHaveTextContent("playlist");
  });

  it("sets audio mode to official", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Official Audio"));
    expect(screen.getByTestId("audioMode")).toHaveTextContent("official");
  });

  it("sets audio mode to raw", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Raw Audio"));
    expect(screen.getByTestId("audioMode")).toHaveTextContent("raw");
  });

  it("sets audio mode to clean", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Clean Audio"));
    expect(screen.getByTestId("audioMode")).toHaveTextContent("clean");
  });

  it("sets download path", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Path"));
    expect(screen.getByTestId("downloadPath")).toHaveTextContent("/downloads");
  });

  it("sets input text", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Input Text"));
    expect(screen.getByTestId("inputText")).toHaveTextContent("song1 song2");
  });

  it("sets progress", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Progress"));
    expect(screen.getByTestId("progress")).toHaveTextContent("50");
  });

  it("sets status", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Status"));
    expect(screen.getByTestId("status")).toHaveTextContent("Downloading...");
  });

  it("sets item count", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Item Count"));
    expect(screen.getByTestId("itemCount")).toHaveTextContent("10");
  });

  it("sets current item", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set Current Item"));
    expect(screen.getByTestId("currentItem")).toHaveTextContent("3");
  });

  it("resets state to defaults", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set URL"));
    fireEvent.click(screen.getByText("Set Audio Mode"));
    fireEvent.click(screen.getByText("Set Official Audio"));
    fireEvent.click(screen.getByText("Set Path"));
    fireEvent.click(screen.getByText("Set Input Text"));
    fireEvent.click(screen.getByText("Reset"));
    expect(screen.getByTestId("url")).toHaveTextContent("");
    expect(screen.getByTestId("mode")).toHaveTextContent("video");
    expect(screen.getByTestId("audioMode")).toHaveTextContent("clean");
    expect(screen.getByTestId("downloadPath")).toHaveTextContent("");
    expect(screen.getByTestId("inputText")).toHaveTextContent("");
  });

  it("throws error when used outside provider", () => {
    expect(() => useDownloadStore()).toThrow();
  });

  it("updates multiple state values sequentially", () => {
    render(
      <DownloadProvider>
        <TestComponent />
      </DownloadProvider>
    );
    fireEvent.click(screen.getByText("Set URL"));
    fireEvent.click(screen.getByText("Set Audio Mode"));
    fireEvent.click(screen.getByText("Set Official Audio"));
    fireEvent.click(screen.getByText("Set Path"));
    expect(screen.getByTestId("url")).toHaveTextContent("https://youtube.com/watch?v=abc");
    expect(screen.getByTestId("mode")).toHaveTextContent("audio");
    expect(screen.getByTestId("audioMode")).toHaveTextContent("official");
    expect(screen.getByTestId("downloadPath")).toHaveTextContent("/downloads");
  });
});
