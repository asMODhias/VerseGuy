import { test, expect } from '@playwright/test';

async function isReachable(url: string) {
  try {
    const r = await fetch(url, { method: 'GET' });
    return r.ok;
  } catch (e) {
    return false;
  }
}

test.describe('Operations E2E', () => {
  test('create operation, add participant and update status', async ({ request }) => {
    const uiUrl = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:3000';
    const apiUrl = process.env.PLAYWRIGHT_API_URL || 'http://127.0.0.1:3001';

    const uiOk = await isReachable(uiUrl + '/');
    if (!uiOk) test.skip(true, `UI not reachable at ${uiUrl}`);

    const apiOk = await isReachable(apiUrl + '/plugins/search');
    if (!apiOk) test.skip(true, `Master server not reachable at ${apiUrl}`);

    const createRes = await request.post(`${apiUrl}/v1/operations`, { data: { name: 'Op-E2E', description: 'Test' } });
    expect(createRes.ok()).toBeTruthy();
    const created = await createRes.json();
    const id = created.id;

    const partRes = await request.post(`${apiUrl}/v1/operations/${id}/participants`, { data: { user_id: 'u1', role: 'eng' } });
    expect(partRes.ok()).toBeTruthy();

    const statusRes = await request.post(`${apiUrl}/v1/operations/${id}/status`, { data: { status: 'Running' } });
    expect(statusRes.ok()).toBeTruthy();

    const gRes = await request.get(`${apiUrl}/v1/operations/${id}`);
    expect(gRes.ok()).toBeTruthy();
    const gJson = await gRes.json();
    expect(gJson.participants.length).toBeGreaterThan(0);
    expect(gJson.status).toBe('Running');
  });
});