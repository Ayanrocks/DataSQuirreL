import { render, screen } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
import SideBarItem from "./SideBarItem.svelte";

describe("SideBarItem Component", () => {
  it("renders correctly for a table item", () => {
    const { container } = render(SideBarItem, {
      entityName: "users",
      isExpanded: false,
      entityType: "Table",
      hasChildren: false,
      fullPath: "db::schema::users",
      toggle: vi.fn(),
      handleTableClick: vi.fn(),
    });

    expect(screen.getByText("users")).toBeInTheDocument();
    expect(container).toMatchSnapshot();
  });

  it("renders correctly for a schema item with children", () => {
    const { container } = render(SideBarItem, {
      entityName: "public",
      isExpanded: false,
      entityType: "Schema",
      hasChildren: true,
      fullPath: "db::public",
      toggle: vi.fn(),
      handleTableClick: vi.fn(),
    });

    expect(screen.getByText("public")).toBeInTheDocument();
    expect(container.querySelector(".expandable-icon")).toBeInTheDocument();
    expect(container).toMatchSnapshot();
  });
});
