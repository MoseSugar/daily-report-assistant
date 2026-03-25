import { useSyncExternalStore } from "react";
import { createDefaultSettings, type AppSettings } from "../models/settings";
import { getSettings, saveSettings, testAiConnection } from "../services/settingsService";

type SettingsStoreState = {
  settings: AppSettings;
  isLoading: boolean;
  isSaving: boolean;
  isTesting: boolean;
  message: string;
  error: string;
};

type Listener = () => void;

const state: SettingsStoreState = {
  settings: createDefaultSettings(),
  isLoading: false,
  isSaving: false,
  isTesting: false,
  message: "",
  error: "",
};

const listeners = new Set<Listener>();

function emitChange() {
  listeners.forEach((listener) => listener());
}

function setState(partial: Partial<SettingsStoreState>) {
  Object.assign(state, partial);
  emitChange();
}

export function setSettings(nextSettings: AppSettings) {
  setState({ settings: nextSettings, error: "", message: "" });
}

export async function loadSettings() {
  setState({ isLoading: true, error: "", message: "" });
  try {
    const settings = await getSettings();
    setState({ settings, isLoading: false });
  } catch (error) {
    setState({
      isLoading: false,
      error: error instanceof Error ? error.message : "读取设置失败",
    });
  }
}

export async function submitSettings() {
  setState({ isSaving: true, error: "", message: "" });
  try {
    const saved = await saveSettings(state.settings);
    setState({
      settings: saved,
      isSaving: false,
      message: "设置已保存并已应用到提醒与快捷键",
    });
  } catch (error) {
    setState({
      isSaving: false,
      error: error instanceof Error ? error.message : "保存设置失败",
    });
  }
}

export async function runAiConnectionTest() {
  setState({ isTesting: true, error: "", message: "" });
  try {
    const result = await testAiConnection(state.settings);
    setState({
      isTesting: false,
      message: result.message,
    });
  } catch (error) {
    setState({
      isTesting: false,
      error: error instanceof Error ? error.message : "AI 连接测试失败",
    });
  }
}

function subscribe(listener: Listener) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function useSettingsStore<T>(
  selector: (value: SettingsStoreState & {
    setSettings: typeof setSettings;
    loadSettings: typeof loadSettings;
    submitSettings: typeof submitSettings;
    runAiConnectionTest: typeof runAiConnectionTest;
  }) => T,
) {
  return useSyncExternalStore(subscribe, () =>
    selector({
      ...state,
      setSettings,
      loadSettings,
      submitSettings,
      runAiConnectionTest,
    }),
  );
}
