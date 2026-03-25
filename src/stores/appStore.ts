import { useSyncExternalStore } from "react";

export const appWindows = [
  { id: "capture", label: "快速记录" },
  { id: "summary", label: "当日汇总" },
  { id: "settings", label: "设置" },
] as const;

export type AppWindowId = (typeof appWindows)[number]["id"];

type AppStoreState = {
  activeWindow: AppWindowId;
  captureFocusVersion: number;
};

type Listener = () => void;

const state: AppStoreState = {
  activeWindow: "capture",
  captureFocusVersion: 0,
};

const listeners = new Set<Listener>();

function emitChange() {
  listeners.forEach((listener) => listener());
}

export function setActiveWindow(windowId: AppWindowId) {
  state.activeWindow = windowId;
  emitChange();
}

export function showSummaryWindow() {
  setActiveWindow("summary");
}

export function showCaptureWindow() {
  setActiveWindow("capture");
}

export function requestCaptureFocus() {
  state.captureFocusVersion += 1;
  emitChange();
}

function subscribe(listener: Listener) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

function getSnapshot() {
  return state;
}

export function useAppStore<T>(selector: (value: AppStoreState & {
  setActiveWindow: typeof setActiveWindow;
  requestCaptureFocus: typeof requestCaptureFocus;
}) => T) {
  return useSyncExternalStore(subscribe, () =>
    selector({ ...getSnapshot(), setActiveWindow, requestCaptureFocus }),
  );
}
