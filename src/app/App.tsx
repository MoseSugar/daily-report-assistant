import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { AppShell } from "../components/AppShell";
import {
  appWindows,
  requestCaptureFocus,
  showCaptureWindow,
  showSummaryWindow,
  useAppStore,
} from "../stores/appStore";
import { CaptureWindow } from "../windows/CaptureWindow";
import { DailySummaryWindow } from "../windows/DailySummaryWindow";
import { SettingsWindow } from "../windows/SettingsWindow";

const windowMap = {
  capture: <CaptureWindow />,
  summary: <DailySummaryWindow />,
  settings: <SettingsWindow />,
};

export default function App() {
  const activeWindow = useAppStore((state) => state.activeWindow);
  const setActiveWindow = useAppStore((state) => state.setActiveWindow);

  useEffect(() => {
    const cleanups: Array<() => void> = [];

    void listen("window.summary.reminder_triggered", () => {
      showSummaryWindow();
    }).then((dispose) => {
      cleanups.push(dispose);
    });

    void listen("window.summary.open_requested", () => {
      showSummaryWindow();
    }).then((dispose) => {
      cleanups.push(dispose);
    });

    void listen("window.capture.hotkey_triggered", () => {
      showCaptureWindow();
      requestCaptureFocus();
    }).then((dispose) => {
      cleanups.push(dispose);
    });

    return () => {
      cleanups.forEach((dispose) => dispose());
    };
  }, []);

  return (
    <AppShell
      activeWindow={activeWindow}
      windows={appWindows}
      onWindowChange={setActiveWindow}
    >
      {windowMap[activeWindow]}
    </AppShell>
  );
}
