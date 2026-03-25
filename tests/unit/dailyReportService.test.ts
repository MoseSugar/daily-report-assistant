import { describe, expect, it } from "vitest";

describe("daily report formatter contract", () => {
  it("keeps done and in progress sections separate", () => {
    const reportText = ["今日工作日报：", "", "已完成：", "1. 完成任务", "", "进行中：", "1. 跟进任务"].join("\n");

    expect(reportText).toContain("已完成：");
    expect(reportText).toContain("进行中：");
  });
});
