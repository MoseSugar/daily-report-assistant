type CaptureFormProps = {
  content: string;
  isSaving: boolean;
  onContentChange: (value: string) => void;
  onSubmit: () => void;
};

export function CaptureForm({
  content,
  isSaving,
  onContentChange,
  onSubmit,
}: CaptureFormProps) {
  return (
    <form
      className="capture-form"
      onSubmit={(event) => {
        event.preventDefault();
        onSubmit();
      }}
    >
      <label htmlFor="task-content">当前任务</label>
      <textarea
        autoFocus
        id="task-content"
        onChange={(event) => onContentChange(event.target.value)}
        placeholder="例如：整理日报助手任务拆解"
        rows={4}
        value={content}
      />
      <button disabled={isSaving} type="submit">
        {isSaving ? "保存中..." : "保存任务"}
      </button>
    </form>
  );
}
