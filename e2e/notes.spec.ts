import { expect, test, type Page } from "@playwright/test";

// The browser half of docs/specs/notes-ui.md — the criteria an integration test
// can't reach, because a page can pass every assertion in `tests/api.rs` and
// still render blank, or silently reload, or post rubbish to the API.
//
// Every check below either stubs the API or seeds its own note, so each one passes
// on its own. That is not a nicety: a test that only passes because an earlier test
// happened to leave data behind is a test you cannot trust the day it goes red.

/**
 * Wait for the page's first fetch to land — either notes are rendered, or the
 * empty state is showing.
 *
 * Do not wait on the list itself. An empty `<ul>` has no height, so the browser
 * calls it invisible, and a check that waits for it to appear will hang on an
 * empty store and pass on a full one. That is exactly the kind of order-dependence
 * that makes a green suite worthless.
 */
async function waitForLoaded(page: Page) {
  await expect
    .poll(async () => {
      const notes = await page.getByTestId("note").count();
      return notes > 0 || (await page.getByTestId("empty-state").isVisible());
    })
    .toBe(true);
}

test("shows_an_empty_state", async ({ page }) => {
  // Stub the list, so this is a check of the empty state itself and not of
  // whatever happens to be in the store when it runs.
  await page.route("**/api/notes", async (route) => {
    if (route.request().method() === "GET") {
      await route.fulfill({ json: [] });
    } else {
      await route.continue();
    }
  });

  await page.goto("/");

  await expect(page.getByTestId("empty-state")).toBeVisible();
  await expect(page.getByTestId("note")).toHaveCount(0);
});

test("shows_existing_notes_on_load", async ({ page, request }) => {
  const text = "a note that was already in the store";
  const seeded = await request.post("/api/notes", { data: { text } });
  expect(seeded.status()).toBe(201);

  await page.goto("/");

  await expect(page.getByTestId("note").filter({ hasText: text })).toBeVisible();
  await expect(page.getByTestId("empty-state")).toBeHidden();
});

test("adds_a_note_without_a_reload", async ({ page }) => {
  await page.goto("/");
  await waitForLoaded(page);

  // A full page reload would wipe this marker. If it survives, the page didn't
  // navigate — the note arrived over fetch, the way the spec asks for.
  await page.evaluate(() => {
    (window as Window & { sameDocument?: boolean }).sameDocument = true;
  });

  const text = "a note typed into the form";
  await page.getByTestId("note-input").fill(text);
  await page.getByTestId("add-note").click();

  await expect(page.getByTestId("note").filter({ hasText: text })).toBeVisible();
  await expect(page.getByTestId("status")).toContainText("Saved");
  await expect(page.getByTestId("note-input")).toHaveValue("");

  const sameDocument = await page.evaluate(
    () => (window as Window & { sameDocument?: boolean }).sameDocument === true,
  );
  expect(sameDocument, "the page reloaded instead of fetching").toBe(true);
});

test("loads_without_errors", async ({ page }) => {
  // Nobody opens the console, which is exactly why a dead asset, a failed request or
  // a thrown handler can sit in a page for months. The browser noticed; nobody asked it.
  //
  // Watch the network as well as the console: a 404 for a missing file is logged by
  // the browser itself, not by page code, so `console` alone never sees it. A check
  // that cannot fail is not a check.
  const problems: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") problems.push(`console error: ${message.text()}`);
  });
  page.on("pageerror", (error) => problems.push(`uncaught: ${error.message}`));
  page.on("requestfailed", (request) => problems.push(`request failed: ${request.url()}`));
  page.on("response", (response) => {
    if (response.status() >= 400) {
      problems.push(`${response.status()}: ${response.url()}`);
    }
  });

  await page.goto("/");
  await waitForLoaded(page);

  expect(problems, "the page did not load cleanly").toEqual([]);
});

test("rejects_an_empty_note_in_the_ui", async ({ page }) => {
  await page.goto("/");
  await waitForLoaded(page);
  const before = await page.getByTestId("note").count();

  let posted = false;
  page.on("request", (request) => {
    if (request.method() === "POST" && request.url().includes("/api/notes")) {
      posted = true;
    }
  });

  await page.getByTestId("note-input").fill("   ");
  await page.getByTestId("add-note").click();

  await expect(page.getByTestId("status")).toHaveText("A note needs some text.");
  await expect(page.getByTestId("note")).toHaveCount(before);
  expect(posted, "an empty note must never reach the API").toBe(false);
});
