type ReportPreviewProps = {
  basicReportText: string;
  polishedReportText?: string | null;
};

export function ReportPreview({
  basicReportText,
  polishedReportText,
}: ReportPreviewProps) {
  const displayText = polishedReportText || basicReportText;

  return (
    <section className="report-preview">
      <h3>{polishedReportText ? "AI 润色日报预览" : "基础日报预览"}</h3>
      {basicReportText && polishedReportText ? (
        <p className="field-hint">当前预览展示 AI 润色结果，复制基础日报仍会使用未润色文本。</p>
      ) : null}
      {displayText ? <pre>{displayText}</pre> : <p>尚未生成基础日报。</p>}
    </section>
  );
}
