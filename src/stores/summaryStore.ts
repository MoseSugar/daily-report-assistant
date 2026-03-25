import { useSyncExternalStore } from "react";
import type { DailyReportDraft } from "../models/report";
import type { TaskStatus } from "../models/task";
import { copyText } from "../services/systemService";
import {
  generateAiPolishedReport,
  generateBasicReport,
  getTodayDraft,
  triggerSummaryReminder,
  updateTask,
} from "../services/summaryService";

type SummaryStoreState = {
  draft: DailyReportDraft | null;
  isLoading: boolean;
  isGenerating: boolean;
  isPolishing: boolean;
  message: string;
  error: string;
};

type Listener = () => void;

const state: SummaryStoreState = {
  draft: null,
  isLoading: false,
  isGenerating: false,
  isPolishing: false,
  message: "",
  error: "",
};

const listeners = new Set<Listener>();

function emitChange() {
  listeners.forEach((listener) => listener());
}

function setState(partial: Partial<SummaryStoreState>) {
  Object.assign(state, partial);
  emitChange();
}

export async function loadTodayDraft() {
  setState({ isLoading: true, error: "", message: "" });
  try {
    const draft = await getTodayDraft();
    setState({ draft, isLoading: false });
  } catch (error) {
    setState({
      isLoading: false,
      error: error instanceof Error ? error.message : "读取汇总失败",
    });
  }
}

export async function saveTaskEdit(taskId: string, status: TaskStatus, note: string) {
  try {
    await updateTask({ taskId, status, note });
    const draft = await getTodayDraft();
    setState({ draft, message: "任务更新成功", error: "" });
  } catch (error) {
    setState({
      error: error instanceof Error ? error.message : "更新任务失败",
      message: "",
    });
  }
}

export async function createBasicReport() {
  if (!state.draft) {
    return;
  }

  setState({ isGenerating: true, error: "", message: "" });
  try {
    const draft = await generateBasicReport(state.draft.date);
    setState({
      draft,
      isGenerating: false,
      message: "基础日报已生成",
    });
  } catch (error) {
    setState({
      isGenerating: false,
      error: error instanceof Error ? error.message : "生成基础日报失败",
    });
  }
}

export async function createAiPolishedReport() {
  if (!state.draft) {
    return;
  }

  setState({ isPolishing: true, error: "", message: "" });
  try {
    const response = await generateAiPolishedReport(state.draft.date);
    setState({
      draft: response.draft,
      isPolishing: false,
      message: response.warningMessage ? "" : "AI 润色完成",
      error: response.warningMessage ?? "",
    });
  } catch (error) {
    setState({
      isPolishing: false,
      error: error instanceof Error ? error.message : "AI 润色失败",
    });
  }
}

async function copyReport(text: string, successMessage: string) {
  try {
    await copyText(text);
    setState({ message: successMessage, error: "" });
  } catch (error) {
    setState({
      error: error instanceof Error ? error.message : "复制日报失败",
      message: "",
    });
  }
}

export async function triggerReminderNow() {
  try {
    const response = await triggerSummaryReminder();
    setState({ message: response.message, error: "" });
  } catch (error) {
    setState({
      error: error instanceof Error ? error.message : "触发提醒失败",
      message: "",
    });
  }
}

export async function copyBasicReport() {
  await copyReport(state.draft?.basicReportText ?? "", "基础日报已复制");
}

export async function copyPolishedReport() {
  await copyReport(state.draft?.polishedReportText ?? "", "AI 润色日报已复制");
}

function subscribe(listener: Listener) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function useSummaryStore<T>(
  selector: (value: SummaryStoreState & {
    loadTodayDraft: typeof loadTodayDraft;
    saveTaskEdit: typeof saveTaskEdit;
    createBasicReport: typeof createBasicReport;
    createAiPolishedReport: typeof createAiPolishedReport;
    copyBasicReport: typeof copyBasicReport;
    copyPolishedReport: typeof copyPolishedReport;
    triggerReminderNow: typeof triggerReminderNow;
  }) => T,
) {
  return useSyncExternalStore(subscribe, () =>
    selector({
      ...state,
      loadTodayDraft,
      saveTaskEdit,
      createBasicReport,
      createAiPolishedReport,
      copyBasicReport,
      copyPolishedReport,
      triggerReminderNow,
    }),
  );
}
