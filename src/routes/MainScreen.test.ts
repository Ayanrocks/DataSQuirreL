import { render, screen } from "@testing-library/svelte";
import { describe, it, expect } from "vitest";
import MainScreen from "./MainScreen.svelte";

describe("MainScreen Component", () => {
  it("renders correctly and mounts sub-components", () => {
    render(MainScreen);

    // Sidebar text or topbar text should be visible (Top bar has class main-top-bar, or similar)
    // We expect to find 'Connections' or 'DataSQuirreL' if there are text labels,
    // or just checking if the main container is present.
    const mainContainer = document.querySelector(".main-container");
    expect(mainContainer).toBeInTheDocument();
    expect(mainContainer).toMatchSnapshot();
  });
});
