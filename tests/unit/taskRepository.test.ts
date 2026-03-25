import { describe, expect, it } from "vitest";
import { createEmptyTaskEntry } from "../../src/models/task";

describe("task model skeleton", () => {
  it("creates a default task entry with done status", () => {
    const entry = createEmptyTaskEntry();

    expect(entry.status).toBe("done");
    expect(entry.note).toBe("");
  });
});
