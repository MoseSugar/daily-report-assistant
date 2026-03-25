import { useSyncExternalStore } from "react";
import type { TaskEntry } from "../models/task";
import { createTask, listTodayTasks } from "../services/captureService";

type CaptureStoreState = {
  content: string;
  tasks: TaskEntry[];
  isSaving: boolean;
  isLoading: boolean;
  message: string;
  error: string;
};

type Listener = () => void;

const state: CaptureStoreState = {
  content: "",
  tasks: [],
  isSaving: false,
  isLoading: false,
  message: "",
  error: "",
};

const listeners = new Set<Listener>();

function emitChange() {
  listeners.forEach((listener) => listener());
}

function setState(partial: Partial<CaptureStoreState>) {
  Object.assign(state, partial);
  emitChange();
}

export function setCaptureContent(content: string) {
  setState({ content, error: "", message: "" });
}

export async function loadTodayTasks() {
  setState({ isLoading: true, error: "", message: "" });
  try {
    const tasks = await listTodayTasks();
    setState({ tasks, isLoading: false });
  } catch (error) {
    setState({
      isLoading: false,
      error: error instanceof Error ? error.message : "读取任务失败",
    });
  }
}

export async function submitCaptureForm() {
  const content = state.content.trim();

  if (!content) {
    setState({ error: "请输入任务内容" });
    return;
  }

  setState({ isSaving: true, error: "", message: "" });

  try {
    const response = await createTask(content);
    const tasks = await listTodayTasks();

    setState({
      content: "",
      tasks,
      isSaving: false,
      message: response.message,
    });
  } catch (error) {
    setState({
      isSaving: false,
      error: error instanceof Error ? error.message : "保存任务失败",
    });
  }
}

function subscribe(listener: Listener) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function useCaptureStore<T>(
  selector: (value: CaptureStoreState & {
    setCaptureContent: typeof setCaptureContent;
    submitCaptureForm: typeof submitCaptureForm;
    loadTodayTasks: typeof loadTodayTasks;
  }) => T,
) {
  return useSyncExternalStore(subscribe, () =>
    selector({
      ...state,
      setCaptureContent,
      submitCaptureForm,
      loadTodayTasks,
    }),
  );
}
