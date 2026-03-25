import type { AppSettings } from "../models/settings";

type SettingsFormProps = {
  settings: AppSettings;
  isSaving: boolean;
  isTesting: boolean;
  onChange: (settings: AppSettings) => void;
  onSave: () => void;
  onTestAiConnection: () => void;
};

export function SettingsForm({
  settings,
  isSaving,
  isTesting,
  onChange,
  onSave,
  onTestAiConnection,
}: SettingsFormProps) {
  return (
    <form
      className="settings-form"
      onSubmit={(event) => {
        event.preventDefault();
        onSave();
      }}
    >
      <section className="settings-section">
        <h3>提醒与快捷键</h3>
        <label className="checkbox-row">
          <input
            checked={settings.reminderEnabled}
            onChange={(event) =>
              onChange({ ...settings, reminderEnabled: event.target.checked })
            }
            type="checkbox"
          />
          启用下班前自动提醒
        </label>
        <label>
          下班时间
          <input
            onChange={(event) =>
              onChange({ ...settings, offWorkTime: event.target.value })
            }
            type="time"
            value={settings.offWorkTime}
          />
        </label>
        <label>
          提前提醒分钟数
          <input
            min={0}
            onChange={(event) =>
              onChange({
                ...settings,
                remindBeforeMinutes: Number(event.target.value) || 0,
              })
            }
            type="number"
            value={settings.remindBeforeMinutes}
          />
        </label>
        <label>
          全局快捷键
          <input
            onChange={(event) =>
              onChange({ ...settings, globalHotkey: event.target.value })
            }
            placeholder="例如：Ctrl+Shift+D"
            value={settings.globalHotkey}
          />
        </label>
        <p className="field-hint">
          支持 `Ctrl`、`Alt`、`Shift` 加主键，主键建议使用字母、数字或 F1-F12。
        </p>
      </section>

      <section className="settings-section">
        <h3>AI 润色</h3>
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
      </section>

      <div className="form-actions">
        <button disabled={isSaving} type="submit">
          {isSaving ? "保存中..." : "保存设置"}
        </button>
        <button disabled={isTesting} onClick={onTestAiConnection} type="button">
          {isTesting ? "测试中..." : "测试 AI 连接"}
        </button>
      </div>
    </form>
  );
}
