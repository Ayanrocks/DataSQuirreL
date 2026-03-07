export type ShortcutConfig = {
  key: string;
  ctrlCmd?: boolean;
  shift?: boolean;
  alt?: boolean;
};

export function isShortcut(e: KeyboardEvent, config: ShortcutConfig): boolean {
  const isCtrlCmd = e.ctrlKey || e.metaKey;
  const isShift = e.shiftKey;
  const isAlt = e.altKey;

  return (
    isCtrlCmd === !!config.ctrlCmd &&
    isShift === !!config.shift &&
    isAlt === !!config.alt &&
    e.key.toLowerCase() === config.key.toLowerCase()
  );
}

export const Shortcuts = {
  Refresh: { key: "r", ctrlCmd: true },
  Save: { key: "s", ctrlCmd: true }, // For future commit shortcut if needed
  Escape: { key: "Escape" },
  Tab: { key: "Tab" },
  Enter: { key: "Enter" },
  Space: { key: " " },
  Delete: { key: "Delete" },
  Backspace: { key: "Backspace" },
};
