import { describe, expect, it } from "vitest";
import { createDefaultSettings } from "../../src/models/settings";

describe("settings model skeleton", () => {
  it("creates the documented default settings", () => {
    const settings = createDefaultSettings();

    expect(settings.globalHotkey).toBe("Ctrl+Shift+D");
    expect(settings.remindBeforeMinutes).toBe(5);
    expect(settings.aiEnabled).toBe(false);
  });
});
