import { render, screen } from "@testing-library/svelte";
import { describe, it, expect } from "vitest";
import ConnectionForm from "./ConnectionForm.svelte";

describe("ConnectionForm Component", () => {
  it("renders form elements properly", () => {
    const { container } = render(ConnectionForm, {
      projectName: "",
      hostName: "",
      port: 5432,
      dbType: "",
      userName: "",
      password: "",
      dbName: "",
      loaderActive: false,
      OnClickConnect: () => {},
    });

    expect(screen.getByText("New Database Connection")).toBeInTheDocument();
    expect(screen.getByLabelText("Project Name")).toBeInTheDocument();
    expect(screen.getByLabelText("Database Type")).toBeInTheDocument();
    expect(screen.getByLabelText("Host")).toBeInTheDocument();
    expect(screen.getByLabelText("Port")).toBeInTheDocument();
    expect(screen.getByLabelText("Username")).toBeInTheDocument();
    expect(screen.getByLabelText("Password")).toBeInTheDocument();
    expect(screen.getByLabelText("Database Name")).toBeInTheDocument();

    // Check if Connect button is present
    expect(
      screen.getByRole("button", { name: /connect/i }),
    ).toBeInTheDocument();

    expect(container).toMatchSnapshot();
  });
});
