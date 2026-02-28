import { render, screen } from "@testing-library/svelte";
import { describe, it, expect } from "vitest";
import SidebarToolbar from "./SidebarToolbar.svelte";

describe("SidebarToolbar Component", () => {
  it("renders correctly", () => {
    const { container } = render(SidebarToolbar);
    expect(screen.getByTitle("Add New Connection")).toBeInTheDocument();
    expect(screen.getByTitle("Refresh")).toBeInTheDocument();
    expect(screen.getByTitle("Disconnect")).toBeInTheDocument();
    expect(screen.getByTitle("Open Query Console")).toBeInTheDocument();
    expect(container).toMatchSnapshot();
  });
});
