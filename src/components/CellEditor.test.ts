import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
import CellEditor from "./CellEditor.svelte";

describe("CellEditor Component", () => {
  it("renders correctly with initial value", () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();

    const { container } = render(CellEditor, {
      initialValue: "Test Value",
      onCommit,
      onCancel,
    });

    const input = screen.getByRole("textbox") as HTMLInputElement;
    expect(input).toBeInTheDocument();
    expect(input.value).toBe("Test Value");
  });

  it("calls onCommit on blur", async () => {
    vi.useFakeTimers();
    const onCommit = vi.fn();
    const onCancel = vi.fn();

    render(CellEditor, {
      initialValue: "Test Value",
      onCommit,
      onCancel,
    });

    const input = screen.getByRole("textbox");
    await fireEvent.blur(input);

    // handleBlur uses setTimeout to defer the commit
    vi.advanceTimersByTime(0);

    expect(onCommit).toHaveBeenCalledWith("Test Value", "none");
    vi.useRealTimers();
  });

  it('calls onCommit with "down" on Enter key', async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();

    render(CellEditor, {
      initialValue: "Test Value",
      onCommit,
      onCancel,
    });

    const input = screen.getByRole("textbox");
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onCommit).toHaveBeenCalledWith("Test Value", "down");
  });

  it('calls onCommit with "next" on Tab key', async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();

    render(CellEditor, {
      initialValue: "Test Value",
      onCommit,
      onCancel,
    });

    const input = screen.getByRole("textbox");
    await fireEvent.keyDown(input, { key: "Tab" });

    expect(onCommit).toHaveBeenCalledWith("Test Value", "next");
  });

  it("calls onCancel on Escape key", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();

    render(CellEditor, {
      initialValue: "Test Value",
      onCommit,
      onCancel,
    });

    const input = screen.getByRole("textbox");
    await fireEvent.keyDown(input, { key: "Escape" });

    expect(onCancel).toHaveBeenCalled();
  });
});
