import { describe, expect, it } from "vitest";
import { createEmptyTaskEntry } from "../../src/models/task";

describe("capture persistence contract", () => {
  it("uses a task shape compatible with reload after restart", () => {
    const entry = createEmptyTaskEntry();

    expect(entry).toHaveProperty("id");
    expect(entry).toHaveProperty("createdAt");
    expect(entry).toHaveProperty("updatedAt");
  });
});
