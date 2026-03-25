import { invokeTauriCommand, tauriCommands } from "./tauriClient";

export async function copyText(text: string): Promise<void> {
  await invokeTauriCommand<{ message: string }>(tauriCommands.system.copyText, {
    payload: { text },
  });
}
