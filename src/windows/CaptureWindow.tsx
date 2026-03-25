import { useEffect } from "react";
import { CaptureForm } from "../components/CaptureForm";
import { ErrorBanner } from "../components/ErrorBanner";
import { useAppStore } from "../stores/appStore";
import { useCaptureStore } from "../stores/captureStore";

export function CaptureWindow() {
  const content = useCaptureStore((state) => state.content);
  const tasks = useCaptureStore((state) => state.tasks);
  const isSaving = useCaptureStore((state) => state.isSaving);
  const isLoading = useCaptureStore((state) => state.isLoading);
  const message = useCaptureStore((state) => state.message);
  const error = useCaptureStore((state) => state.error);
  const setCaptureContent = useCaptureStore((state) => state.setCaptureContent);
  const submitCaptureForm = useCaptureStore((state) => state.submitCaptureForm);
  const loadTodayTasks = useCaptureStore((state) => state.loadTodayTasks);
  const captureFocusVersion = useAppStore((state) => state.captureFocusVersion);

  useEffect(() => {
    void loadTodayTasks();
  }, [loadTodayTasks]);

  useEffect(() => {
    const field = document.getElementById("task-content");
    if (field instanceof HTMLTextAreaElement) {
      field.focus();
      field.select();
    }
  }, [captureFocusVersion]);

  return (
    <section>
      <h2>快速记录窗口</h2>
      <p>输入当前任务并保存到本地 SQLite，已保存任务会在下方即时显示。</p>
      <CaptureForm
        content={content}
        isSaving={isSaving}
        onContentChange={setCaptureContent}
        onSubmit={() => void submitCaptureForm()}
      />
      {message ? <p className="feedback success">{message}</p> : null}
      {error ? <ErrorBanner error={error} /> : null}
      <section className="saved-task-list">
        <h3>今日已保存任务</h3>
        {isLoading ? <p>正在读取任务...</p> : null}
        {!isLoading && tasks.length === 0 ? <p>今天还没有保存任何任务。</p> : null}
        {!isLoading && tasks.length > 0 ? (
          <ol>
            {tasks.map((task) => (
              <li key={task.id}>
                <strong>{task.content}</strong>
                <div className="saved-task-meta">
                  <span>状态：{task.status === "done" ? "已完成" : "进行中"}</span>
                  <span>时间：{new Date(task.createdAt).toLocaleString()}</span>
                </div>
              </li>
            ))}
          </ol>
        ) : null}
      </section>
    </section>
  );
}
