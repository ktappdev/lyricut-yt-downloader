import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { DownloadInput } from "../components/layout/MainLayout";
import "@testing-library/jest-dom";

describe("DownloadInput Component", () => {
  it("renders textarea with correct placeholder", () => {
    render(<DownloadInput value="" onChange={() => {}} />);
    expect(screen.getByPlaceholderText("Paste links or song names (one per line)...")).toBeInTheDocument();
  });

  it("renders textarea with custom placeholder", () => {
    render(<DownloadInput value="" onChange={() => {}} placeholder="Enter URLs here..." />);
    expect(screen.getByPlaceholderText("Enter URLs here...")).toBeInTheDocument();
  });

  it("displays the value correctly", () => {
    render(<DownloadInput value="https://youtube.com/watch?v=abc123" onChange={() => {}} />);
    expect(screen.getByDisplayValue("https://youtube.com/watch?v=abc123")).toBeInTheDocument();
  });

  it("renders textarea with minimum height", () => {
    const { container } = render(<DownloadInput value="" onChange={() => {}} />);
    const textarea = container.querySelector("textarea");
    // Updated to match new min-h-[140px] class
    expect(textarea).toHaveClass("min-h-[140px]");
  });

  it("allows resizing vertically", () => {
    const { container } = render(<DownloadInput value="" onChange={() => {}} />);
    const textarea = container.querySelector("textarea");
    expect(textarea).toHaveClass("resize-y");
  });

  it("applies font-mono class", () => {
    const { container } = render(<DownloadInput value="" onChange={() => {}} />);
    const textarea = container.querySelector("textarea");
    expect(textarea).toHaveClass("font-mono");
  });

  it("applies text-sm class", () => {
    const { container } = render(<DownloadInput value="" onChange={() => {}} />);
    const textarea = container.querySelector("textarea");
    expect(textarea).toHaveClass("text-sm");
  });

  it("calls onChange when textarea value changes", () => {
    const handleChange = vi.fn();
    render(<DownloadInput value="" onChange={handleChange} />);
    fireEvent.change(screen.getByPlaceholderText("Paste links or song names (one per line)..."), {
      target: { value: "New value" },
    });
    expect(handleChange).toHaveBeenCalledWith("New value");
  });

  it("calls onChange with multiple lines", () => {
    const handleChange = vi.fn();
    render(<DownloadInput value="" onChange={handleChange} />);
    const multilineText = "Line 1\nLine 2\nLine 3";
    fireEvent.change(screen.getByPlaceholderText("Paste links or song names (one per line)..."), {
      target: { value: multilineText },
    });
    expect(handleChange).toHaveBeenCalledWith(multilineText);
  });

  it("disables textarea when disabled prop is set", () => {
    render(<DownloadInput value="" onChange={() => {}} disabled />);
    const textarea = screen.getByPlaceholderText("Paste links or song names (one per line)...");
    expect(textarea).toBeDisabled();
  });

  it("applies custom className", () => {
    const { container } = render(<DownloadInput value="" onChange={() => {}} className="custom-class" />);
    const textarea = container.querySelector("textarea");
    expect(textarea).toHaveClass("custom-class");
  });

  it("handles empty value", () => {
    const handleChange = vi.fn();
    render(<DownloadInput value="" onChange={handleChange} />);
    expect(screen.getByPlaceholderText("Paste links or song names (one per line)...")).toHaveValue("");
  });

  it("handles very long input", () => {
    const longInput = "A".repeat(10000);
    render(<DownloadInput value={longInput} onChange={() => {}} />);
    expect(screen.getByDisplayValue(longInput)).toBeInTheDocument();
  });

  it("handles special characters in input", () => {
    const { container } = render(<DownloadInput value="https://youtu.be/xyz789" onChange={() => {}} />);
    const textarea = container.querySelector("textarea");
    expect(textarea).toHaveValue("https://youtu.be/xyz789");
  });
});
