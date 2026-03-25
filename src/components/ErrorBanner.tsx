type ErrorBannerProps = {
  error: string;
};

function getFriendlyErrorMessage(error: string) {
  const normalized = error.toLowerCase();

  if (normalized.includes("global hotkey")) {
    return "全局快捷键注册失败，可能与系统或其他应用冲突，请更换组合键后重试。";
  }

  if (normalized.includes("clipboard")) {
    return "复制失败，请稍后重试；如仍失败，可先手动复制结果文本。";
  }

  if (normalized.includes("ai request returned http")) {
    return error.replace("internal error: ", "");
  }

  if (normalized.includes("ai response parse failed")) {
    return error.replace("internal error: ", "");
  }

  if (normalized.includes("ai response choices are empty")) {
    return "AI 返回成功响应，但 choices 为空，请检查当前兼容接口或模型返回格式。";
  }

  if (normalized.includes("ai response message content is missing")) {
    return "AI 返回成功响应，但缺少 message.content 字段，请检查当前兼容接口返回格式。";
  }

  if (normalized.includes("ai_base_url")) {
    return "启用 AI 润色前，请先填写 Base URL。";
  }

  if (normalized.includes("ai_model")) {
    return "启用 AI 润色前，请先填写模型名。";
  }

  if (normalized.includes("ai_api_key_ref")) {
    return "启用 AI 润色前，请先填写 API Key。";
  }

  if (normalized.includes("off_work_time")) {
    return "请填写合法的下班时间，格式示例为 18:00。";
  }

  if (normalized.includes("copy text must not be empty")) {
    return "当前没有可复制的日报内容，请先生成日报。";
  }

  return error;
}

export function ErrorBanner({ error }: ErrorBannerProps) {
  return (
    <div className="error-banner" role="alert">
      {getFriendlyErrorMessage(error)}
    </div>
  );
}

export { getFriendlyErrorMessage };
