import type { AppSettings } from "../models/settings";

type AiSettingsFormProps = {
  settings: AppSettings;
  isSaving: boolean;
  onChange: (settings: AppSettings) => void;
  onSave: () => void;
};

export function AiSettingsForm({
  settings,
  isSaving,
  onChange,
  onSave,
}: AiSettingsFormProps) {
  return (
    <form
      className="settings-form"
      onSubmit={(event) => {
        event.preventDefault();
        onSave();
      }}
    >
      <label className="checkbox-row">
        <input
          checked={settings.aiEnabled}
          onChange={(event) =>
            onChange({ ...settings, aiEnabled: event.target.checked })
          }
          type="checkbox"
        />
        启用 AI 润色
      </label>
      <label>
        Base URL
        <input
          onChange={(event) =>
            onChange({ ...settings, aiBaseUrl: event.target.value })
          }
          placeholder="例如：https://dashscope.aliyuncs.com/compatible-mode/v1"
          value={settings.aiBaseUrl}
        />
      </label>
      <label>
        API Key
        <input
          onChange={(event) =>
            onChange({ ...settings, aiApiKeyRef: event.target.value })
          }
          placeholder="输入兼容接口的 API Key"
          type="password"
          value={settings.aiApiKeyRef}
        />
      </label>
      <label>
        模型名
        <input
          onChange={(event) =>
            onChange({ ...settings, aiModel: event.target.value })
          }
          placeholder="例如：qwen-plus / gpt-4o-mini"
          value={settings.aiModel}
        />
      </label>
      <label>
        可选系统提示词
        <textarea
          onChange={(event) =>
            onChange({ ...settings, aiSystemPrompt: event.target.value })
          }
          placeholder="例如：保持专业、避免夸大，适合企业 IM 发送"
          rows={4}
          value={settings.aiSystemPrompt}
        />
      </label>
      <button disabled={isSaving} type="submit">
        {isSaving ? "保存中..." : "保存 AI 配置"}
      </button>
    </form>
  );
}
