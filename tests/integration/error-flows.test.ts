import { describe, expect, it } from "vitest";
import { getFriendlyErrorMessage } from "../../src/components/ErrorBanner";

describe("error flow messages", () => {
  it("maps clipboard failures to a user-facing hint", () => {
    expect(getFriendlyErrorMessage("clipboard unavailable: denied")).toContain("复制失败");
  });

  it("maps hotkey registration failures to a conflict hint", () => {
    expect(
      getFriendlyErrorMessage("global hotkey registration failed: AlreadyRegistered"),
    ).toContain("全局快捷键注册失败");
  });
});
