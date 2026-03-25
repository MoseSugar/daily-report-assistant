import { useEffect } from "react";
import { EmptyState } from "../components/EmptyState";
import { ErrorBanner } from "../components/ErrorBanner";
import { ReportActions } from "../components/ReportActions";
import { ReportPreview } from "../components/ReportPreview";
import { TaskList } from "../components/TaskList";
import { useSummaryStore } from "../stores/summaryStore";

export function DailySummaryWindow() {
  const draft = useSummaryStore((state) => state.draft);
  const isLoading = useSummaryStore((state) => state.isLoading);
  const isGenerating = useSummaryStore((state) => state.isGenerating);
  const isPolishing = useSummaryStore((state) => state.isPolishing);
  const message = useSummaryStore((state) => state.message);
  const error = useSummaryStore((state) => state.error);
  const loadTodayDraft = useSummaryStore((state) => state.loadTodayDraft);
  const saveTaskEdit = useSummaryStore((state) => state.saveTaskEdit);
  const createBasicReport = useSummaryStore((state) => state.createBasicReport);
  const createAiPolishedReport = useSummaryStore((state) => state.createAiPolishedReport);
  const copyBasicReport = useSummaryStore((state) => state.copyBasicReport);
  const copyPolishedReport = useSummaryStore((state) => state.copyPolishedReport);
  const triggerReminderNow = useSummaryStore((state) => state.triggerReminderNow);

  useEffect(() => {
    void loadTodayDraft();
  }, [loadTodayDraft]);

  return (
    <section>
      <h2>当日汇总窗口</h2>
      <p>查看今日任务、修改状态与备注，并生成不依赖 AI 的基础日报。</p>
      <ReportActions
        hasBasicReport={Boolean(draft?.basicReportText)}
        hasDraft={Boolean(draft)}
        hasPolishedReport={Boolean(draft?.polishedReportText)}
        isGenerating={isGenerating}
        isPolishing={isPolishing}
        onCopyBasic={() => void copyBasicReport()}
        onCopyPolished={() => void copyPolishedReport()}
        onGenerateBasic={() => void createBasicReport()}
        onPolishWithAi={() => void createAiPolishedReport()}
        onTriggerReminder={() => void triggerReminderNow()}
      />
      {message ? <p className="feedback success">{message}</p> : null}
      {error ? <ErrorBanner error={error} /> : null}
      {isLoading ? <p>正在读取今日任务...</p> : null}
      {!isLoading && draft && draft.entries.length === 0 ? (
        <EmptyState
          description="可以先到快速记录窗口补录任务，再回来生成基础日报。"
          title="今天还没有任务"
        />
      ) : null}
      {!isLoading && draft && draft.entries.length > 0 ? (
        <>
          <TaskList
            onSaveTask={async (taskId, status, note) => {
              await saveTaskEdit(taskId, status, note);
            }}
            tasks={draft.entries}
          />
          <ReportPreview
            basicReportText={draft.basicReportText}
            polishedReportText={draft.polishedReportText}
          />
        </>
      ) : null}
    </section>
  );
}
