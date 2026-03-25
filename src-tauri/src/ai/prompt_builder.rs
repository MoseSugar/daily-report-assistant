use crate::domain::daily_report_draft::DailyReportDraft;

pub fn build_system_prompt(custom_prompt: &str) -> String {
    let default_prompt = "你是一个工作日报润色助手。输出必须自然、简洁、专业，明确区分已完成和进行中，尽量保留原始任务信息，不要夸大未提供的事实。";

    if custom_prompt.trim().is_empty() {
        default_prompt.to_string()
    } else {
        format!("{default_prompt}\n额外要求：{}", custom_prompt.trim())
    }
}

pub fn build_user_prompt(draft: &DailyReportDraft) -> String {
    format!(
        "请基于以下基础日报进行润色，保持可直接复制发送：\n\n日期：{}\n\n基础日报：\n{}",
        draft.date, draft.basic_report_text
    )
}
