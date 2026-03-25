import { describe, expect, it } from "vitest";
import { createEmptyDraft } from "../../src/models/report";

describe("summary empty state contract", () => {
  it("creates a draft with no entries and no generated report", () => {
    const draft = createEmptyDraft("2026-03-24");

    expect(draft.entries).toHaveLength(0);
    expect(draft.basicReportText).toBe("");
    expect(draft.polishedReportText).toBeNull();
  });
});
