import type { AppSettings } from "../models/settings";
import { invokeTauriCommand, tauriCommands } from "./tauriClient";

type AiConnectionCheckResult = {
  message: string;
};

export async function getSettings(): Promise<AppSettings> {
  return invokeTauriCommand<AppSettings>(tauriCommands.settings.get);
}

export async function saveSettings(settings: AppSettings): Promise<AppSettings> {
  return invokeTauriCommand<AppSettings>(tauriCommands.settings.save, { settings });
}

export async function testAiConnection(
  settings: AppSettings,
): Promise<AiConnectionCheckResult> {
  return invokeTauriCommand<AiConnectionCheckResult>(
    tauriCommands.settings.testAiConnection,
    { settings },
  );
}
