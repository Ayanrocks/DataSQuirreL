import { render, screen } from "@testing-library/svelte";
import { describe, it, expect } from "vitest";
import MainTopBar from "./MainTopBar.svelte";

describe("MainTopBar Component", () => {
  it("renders correctly with given connection name", () => {
    const { container } = render(MainTopBar, {
      connectionName: "TestConnection123",
    });

    // Check if title consists of connection name
    expect(
      screen.getByText("TestConnection123 - DataSquirrel"),
    ).toBeInTheDocument();

    expect(container).toMatchSnapshot();
  });
});
