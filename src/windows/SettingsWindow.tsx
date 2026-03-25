import { useEffect } from "react";
import { ErrorBanner } from "../components/ErrorBanner";
import { SettingsForm } from "../components/SettingsForm";
import { useSettingsStore } from "../stores/settingsStore";

export function SettingsWindow() {
  const settings = useSettingsStore((state) => state.settings);
  const isLoading = useSettingsStore((state) => state.isLoading);
  const isSaving = useSettingsStore((state) => state.isSaving);
  const isTesting = useSettingsStore((state) => state.isTesting);
  const message = useSettingsStore((state) => state.message);
  const error = useSettingsStore((state) => state.error);
  const setSettings = useSettingsStore((state) => state.setSettings);
  const loadSettings = useSettingsStore((state) => state.loadSettings);
  const submitSettings = useSettingsStore((state) => state.submitSettings);
  const runAiConnectionTest = useSettingsStore((state) => state.runAiConnectionTest);

  useEffect(() => {
    void loadSettings();
  }, [loadSettings]);

  return (
    <section>
      <h2>设置窗口</h2>
      <p>配置提醒时间、全局快捷键，以及通用 OpenAI 兼容接口所需的 AI 参数。</p>
      {isLoading ? <p>正在读取设置...</p> : null}
      {message ? <p className="feedback success">{message}</p> : null}
      {error ? <ErrorBanner error={error} /> : null}
      <SettingsForm
        isSaving={isSaving}
        isTesting={isTesting}
        onChange={setSettings}
        onSave={() => void submitSettings()}
        onTestAiConnection={() => void runAiConnectionTest()}
        settings={settings}
      />
    </section>
  );
}
