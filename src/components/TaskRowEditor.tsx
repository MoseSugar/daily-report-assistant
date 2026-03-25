import { useState } from "react";
import type { TaskEntry, TaskStatus } from "../models/task";

type TaskRowEditorProps = {
  task: TaskEntry;
  onSave: (taskId: string, status: TaskStatus, note: string) => Promise<void>;
};

export function TaskRowEditor({ task, onSave }: TaskRowEditorProps) {
  const [status, setStatus] = useState<TaskStatus>(task.status);
  const [note, setNote] = useState(task.note);
  const [isSaving, setIsSaving] = useState(false);

  return (
    <li className="task-row-editor">
      <div className="task-row-header">
        <strong>{task.content}</strong>
        <span>{new Date(task.createdAt).toLocaleString()}</span>
      </div>
      <div className="task-row-fields">
        <label>
          状态
          <select
            onChange={(event) => setStatus(event.target.value as TaskStatus)}
            value={status}
          >
            <option value="done">已完成</option>
            <option value="in_progress">进行中</option>
          </select>
        </label>
        <label>
          备注
          <input
            onChange={(event) => setNote(event.target.value)}
            placeholder="补充进展或说明"
            value={note}
          />
        </label>
        <button
          disabled={isSaving}
          onClick={async () => {
            setIsSaving(true);
            try {
              await onSave(task.id, status, note);
            } finally {
              setIsSaving(false);
            }
          }}
          type="button"
        >
          {isSaving ? "保存中..." : "保存编辑"}
        </button>
      </div>
    </li>
  );
}
