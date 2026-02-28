import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
import RecentProjects from "./RecentProjects.svelte";

describe("RecentProjects Component", () => {
  it("renders empty state correctly", () => {
    const { container } = render(RecentProjects, {
      projects: [],
      onConnect: vi.fn(),
      onEdit: vi.fn(),
      onDelete: vi.fn(),
      recentProjectsLoading: false,
    });

    expect(screen.getByText("No recent Project")).toBeInTheDocument();
    expect(container).toMatchSnapshot();
  });

  it("renders projects correctly and handles clicks", async () => {
    const mockProjects = [
      { id: "1", name: "Project Alpha" },
      { id: "2", name: "Project Beta" },
    ] as any;

    const onConnect = vi.fn();
    const onEdit = vi.fn();
    const onDelete = vi.fn();

    // @ts-ignore
    const { container } = render(RecentProjects, {
      projects: mockProjects,
      onConnect,
      onEdit,
      onDelete,
      recentProjectsLoading: false,
    });

    expect(screen.getByText("Project Alpha")).toBeInTheDocument();
    expect(screen.getByText("Project Beta")).toBeInTheDocument();

    const connectBtns = screen.getAllByTitle("Connect to Project");
    expect(connectBtns).toHaveLength(2);

    await fireEvent.click(connectBtns[0]);
    expect(onConnect).toHaveBeenCalledWith(mockProjects[0]);

    const deleteBtns = screen.getAllByTitle("Delete Project");
    await fireEvent.click(deleteBtns[1]);
    expect(onDelete).toHaveBeenCalledWith(mockProjects[1]);

    expect(container).toMatchSnapshot();
  });
});
