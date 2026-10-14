import { backend } from "./bridge";
import { button, element } from "./dom";

const TESTS: ReadonlyArray<[string, string]> = [
  ["follower", "New follower"],
  ["comment", "Comment reply"],
  ["tip", "Buzz tip"],
  ["milestone", "Reaction milestone"],
  ["model", "Model update"],
  ["collection", "Collection update"],
  ["burst", "3 at once"],
];
const MAX_LOG_LINES = 50;

/** Detached window: fires fake notifications through the real toast and sound path. */
export function renderTestPanel(root: HTMLElement): void {
  const shell = element("div", "shell test-shell");
  shell.append(element("h2", "test-title", "Notification test panel"));
  shell.append(element("p", "help", "Each button sends a fake notification through the same toast and sound path as a real sync, using your current settings. This window is independent: you can minimize the main window."));

  const log = element("div", "test-log");
  const write = (message: string): void => {
    const time = new Date().toLocaleTimeString();
    log.prepend(element("div", "", `${time}  ${message}`));
    while (log.childElementCount > MAX_LOG_LINES) log.lastElementChild?.remove();
  };

  const grid = element("div", "test-grid");
  for (const [kind, label] of TESTS) {
    grid.append(button("secondary", label, () => {
      backend.sendTestNotification(kind)
        .then((result) => write(`${label}: ${result}`))
        .catch((error: unknown) => write(`${label}: failed (${typeof error === "string" ? error : "error"})`));
    }));
  }
  shell.append(grid, log, button("secondary full", "Clear log", () => log.replaceChildren()));
  root.replaceChildren(shell);
}
