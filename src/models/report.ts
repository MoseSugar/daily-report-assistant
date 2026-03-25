import type { TaskEntry } from "./task";

export type ReportGenerationSource = "basic" | "ai";

export type DailyReportDraft = {
  date: string;
  entries: TaskEntry[];
  doneEntries: TaskEntry[];
  inProgressEntries: TaskEntry[];
  basicReportText: string;
  polishedReportText: string | null;
  generationSource: ReportGenerationSource;
  lastGeneratedAt: string | null;
};

export function createEmptyDraft(date = ""): DailyReportDraft {
  return {
    date,
    entries: [],
    doneEntries: [],
    inProgressEntries: [],
    basicReportText: "",
    polishedReportText: null,
    generationSource: "basic",
    lastGeneratedAt: null,
  };
}
