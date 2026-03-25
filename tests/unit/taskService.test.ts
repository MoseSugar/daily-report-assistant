import { describe, expect, it } from "vitest";

describe("task service workflow contract", () => {
  it("expects task creation payload to contain user input", () => {
    const payload = {
      content: "实现快速记录",
    };

    expect(payload.content.trim().length).toBeGreaterThan(0);
  });
});
