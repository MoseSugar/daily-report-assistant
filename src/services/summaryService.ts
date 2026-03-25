import type { DailyReportDraft } from "../models/report";
import type { TaskEntry, TaskStatus } from "../models/task";
import { invokeTauriCommand, tauriCommands } from "./tauriClient";

type UpdateTaskPayload = {
  taskId: string;
  status: TaskStatus;
  note: string;
};

export type AiPolishResponse = {
  draft: DailyReportDraft;
  warningMessage: string | null;
};

export async function getTodayDraft(): Promise<DailyReportDraft> {
  return invokeTauriCommand<DailyReportDraft>(tauriCommands.summary.getTodayDraft);
}

export async function updateTask(payload: UpdateTaskPayload): Promise<TaskEntry> {
  return invokeTauriCommand<TaskEntry>(tauriCommands.summary.updateTask, { payload });
}

export async function generateBasicReport(date: string): Promise<DailyReportDraft> {
  return invokeTauriCommand<DailyReportDraft>(
    tauriCommands.summary.generateBasicReport,
    { payload: { date } },
  );
}

export async function triggerSummaryReminder(): Promise<{ message: string }> {
  return invokeTauriCommand<{ message: string }>("system_trigger_summary_reminder");
}

export async function generateAiPolishedReport(
  date: string,
): Promise<AiPolishResponse> {
  return invokeTauriCommand<AiPolishResponse>(
    tauriCommands.summary.generateAiPolishedReport,
    { payload: { date } },
  );
}
