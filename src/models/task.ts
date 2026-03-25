export type TaskStatus = "done" | "in_progress";

export type TaskEntry = {
  id: string;
  date: string;
  createdAt: string;
  updatedAt: string;
  content: string;
  status: TaskStatus;
  note: string;
};

export function createEmptyTaskEntry(): TaskEntry {
  return {
    id: "",
    date: "",
    createdAt: "",
    updatedAt: "",
    content: "",
    status: "done",
    note: "",
  };
}
