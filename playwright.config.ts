import { defineConfig, devices } from "@playwright/test";
import { rmSync } from "node:fs";

// The browser suite drives the real service, backed by a scratch notes file.
// Wipe it first, so a leftover note from the last run can't change what the
// page shows this run.
const NOTES_FILE = ".e2e-notes.json";
rmSync(NOTES_FILE, { force: true });

export default defineConfig({
  testDir: "./e2e",
  // One server, one store: run the checks one at a time so they can't race.
  workers: 1,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? "line" : "list",
  use: {
    baseURL: "http://127.0.0.1:3000",
    trace: "on-first-retry",
  },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"] } }],
  webServer: {
    command: "cargo run --quiet",
    url: "http://127.0.0.1:3000/healthz",
    env: { NOTES_FILE },
    // Never reuse a server we didn't start — its store could hold anything.
    reuseExistingServer: false,
    timeout: 180_000,
  },
});
