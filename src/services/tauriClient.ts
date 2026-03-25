import { invoke } from "@tauri-apps/api/core";

export async function invokeTauriCommand<TResponse>(
  command: string,
  payload?: Record<string, unknown>,
): Promise<TResponse> {
  return invoke<TResponse>(command, payload);
}

export const tauriCommands = {
  capture: {
    createTask: "capture_create_task",
    listTodayTasks: "capture_list_today_tasks",
  },
  summary: {
    getTodayDraft: "summary_get_today_draft",
    updateTask: "summary_update_task",
    generateBasicReport: "summary_generate_basic_report",
    generateAiPolishedReport: "summary_generate_ai_polished_report",
  },
  settings: {
    get: "settings_get",
    save: "settings_save",
    testAiConnection: "settings_test_ai_connection",
  },
  system: {
    ping: "system_ping",
    copyText: "system_copy_text",
    showCaptureWindow: "system_show_capture_window",
    showSummaryWindow: "system_show_summary_window",
  },
} as const;
