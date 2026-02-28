import { render, screen } from "@testing-library/svelte";
import { describe, it, expect } from "vitest";
import Tabs from "./Tabs.svelte";

describe("Tabs Component", () => {
  it("renders correctly", () => {
    const { container } = render(Tabs, { name: "Settings" });
    expect(screen.getByText("Settings Tab Content")).toBeInTheDocument();
    expect(
      screen.getByText("This is the content for the Settings tab."),
    ).toBeInTheDocument();
    expect(container).toMatchSnapshot();
  });
});
