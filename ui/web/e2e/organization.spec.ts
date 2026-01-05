import { test, expect } from '@playwright/test';
import { spawn } from 'child_process';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';

// NOTE: This test assumes you have the web dev server ("npm start") running on http://127.0.0.1:3000
// and the master server running on port 3001 (set MASTER_SERVER_PORT=3001 when starting master-server).
// If the services are not running, the test will skip (so it is safe to run in environments where dev server isn't started).

async function isReachable(url: string) {
  try {
    const r = await fetch(url, { method: 'GET' });
    return r.ok;
  } catch (e) {
    return false;
  }
}

test.describe('Organization E2E', () => {
  test('create org through UI and verify listing', async ({ page, request }) => {
    const uiUrl = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:3000';
    const apiUrl = process.env.PLAYWRIGHT_API_URL || 'http://127.0.0.1:3001';

    // Quick reachability check; skip if not running
    const uiOk = await isReachable(uiUrl + '/');
    if (!uiOk) test.skip(true, `UI not reachable at ${uiUrl} — run 'npm start' in ui/web to run this test`);

    const apiOk = await isReachable(apiUrl + '/plugins/search');
    if (!apiOk) test.skip(true, `Master server not reachable at ${apiUrl} — start with MASTER_SERVER_PORT=3001 cargo run -p master_server --features run-server`);

    await page.goto(uiUrl);
    await expect(page.locator('h1')).toHaveText('Organization');

    // Fill create form
    await page.fill('input[placeholder="Name"]', 'E2E Test Org');
    await page.fill('input[placeholder="Tag"]', 'E2E');
    await page.click('button:has-text("Create")');

    // Wait for the created org to appear in the table
    await expect(page.locator('table')).toContainText('E2E Test Org');
  });
});