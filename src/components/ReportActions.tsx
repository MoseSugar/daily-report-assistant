type ReportActionsProps = {
  hasDraft: boolean;
  hasBasicReport: boolean;
  hasPolishedReport: boolean;
  isGenerating: boolean;
  isPolishing: boolean;
  onGenerateBasic: () => void;
  onPolishWithAi: () => void;
  onCopyBasic: () => void;
  onCopyPolished: () => void;
  onTriggerReminder: () => void;
};

export function ReportActions({
  hasDraft,
  hasBasicReport,
  hasPolishedReport,
  isGenerating,
  isPolishing,
  onGenerateBasic,
  onPolishWithAi,
  onCopyBasic,
  onCopyPolished,
  onTriggerReminder,
}: ReportActionsProps) {
  return (
    <div className="summary-actions">
      <button disabled={isGenerating || !hasDraft} onClick={onGenerateBasic} type="button">
        {isGenerating ? "生成中..." : "生成基础日报"}
      </button>
      <button disabled={isPolishing || !hasDraft} onClick={onPolishWithAi} type="button">
        {isPolishing ? "润色中..." : "AI 润色"}
      </button>
      <button disabled={!hasBasicReport} onClick={onCopyBasic} type="button">
        复制基础日报
      </button>
      <button disabled={!hasPolishedReport} onClick={onCopyPolished} type="button">
        复制 AI 润色日报
      </button>
      <button onClick={onTriggerReminder} type="button">
        模拟提醒弹窗
      </button>
    </div>
  );
}
