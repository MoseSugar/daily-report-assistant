export type AppSettings = {
  reminderEnabled: boolean;
  offWorkTime: string;
  remindBeforeMinutes: number;
  globalHotkey: string;
  aiEnabled: boolean;
  aiBaseUrl: string;
  aiModel: string;
  aiApiKeyRef: string;
  aiSystemPrompt: string;
  reportTemplate: string;
  updatedAt: string;
};

export function createDefaultSettings(): AppSettings {
  return {
    reminderEnabled: true,
    offWorkTime: "18:00",
    remindBeforeMinutes: 5,
    globalHotkey: "Ctrl+Shift+D",
    aiEnabled: false,
    aiBaseUrl: "",
    aiModel: "",
    aiApiKeyRef: "",
    aiSystemPrompt: "",
    reportTemplate: "default",
    updatedAt: new Date(0).toISOString(),
  };
}
