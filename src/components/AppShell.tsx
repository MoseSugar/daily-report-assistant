import type { ReactNode } from "react";
import type { AppWindowId } from "../stores/appStore";

type AppShellProps = {
  activeWindow: AppWindowId;
  windows: ReadonlyArray<{ id: AppWindowId; label: string }>;
  onWindowChange: (windowId: AppWindowId) => void;
  children: ReactNode;
};

export function AppShell({
  activeWindow,
  windows,
  onWindowChange,
  children,
}: AppShellProps) {
  return (
    <div className="app-shell">
      <header className="app-header">
        <div>
          <h1>日报助手</h1>
          <p>通过快速记录、汇总编辑、AI 润色与复制，完成下班前日报闭环。</p>
        </div>
        <nav className="window-nav" aria-label="window navigation">
          {windows.map((windowItem) => (
            <button
              key={windowItem.id}
              className={windowItem.id === activeWindow ? "active" : ""}
              onClick={() => onWindowChange(windowItem.id)}
              type="button"
            >
              {windowItem.label}
            </button>
          ))}
        </nav>
      </header>
      <main className="window-panel">{children}</main>
    </div>
  );
}
