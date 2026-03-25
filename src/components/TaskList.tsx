import type { TaskEntry, TaskStatus } from "../models/task";
import { TaskRowEditor } from "./TaskRowEditor";

type TaskListProps = {
  tasks: TaskEntry[];
  onSaveTask: (taskId: string, status: TaskStatus, note: string) => Promise<void>;
};

export function TaskList({ tasks, onSaveTask }: TaskListProps) {
  return (
    <ol className="task-list">
      {tasks.map((task) => (
        <TaskRowEditor key={task.id} onSave={onSaveTask} task={task} />
      ))}
    </ol>
  );
}
