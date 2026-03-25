import type { TaskEntry } from "../models/task";
import { invokeTauriCommand, tauriCommands } from "./tauriClient";

type CreateTaskResponse = {
  task: TaskEntry;
  message: string;
};

export async function createTask(content: string): Promise<CreateTaskResponse> {
  return invokeTauriCommand<CreateTaskResponse>(tauriCommands.capture.createTask, {
    payload: { content },
  });
}

export async function listTodayTasks(): Promise<TaskEntry[]> {
  return invokeTauriCommand<TaskEntry[]>(tauriCommands.capture.listTodayTasks);
}
