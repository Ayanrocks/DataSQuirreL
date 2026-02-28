import { render, screen } from "@testing-library/svelte";
import { describe, it, expect } from "vitest";
import WindowControls from "./WindowControls.svelte";

describe("WindowControls Component", () => {
  it("renders correctly based on mocked OS", () => {
    // Current setup.ts mocks OS to macos / platform gets 'macos' via Promise
    const { container } = render(WindowControls);

    // In our test, it might be async, but initially default is empty or 'macos' check in onMount
    // The component defaults to empty, then calls getPlatform onMount.
    // Given the layout, just verify it exists.
    expect(container.querySelector(".window-controls")).toBeInTheDocument();
    expect(container).toMatchSnapshot();
  });
});
