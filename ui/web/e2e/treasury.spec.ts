import { test, expect } from '@playwright/test';

async function isReachable(url: string) {
  try {
    const r = await fetch(url, { method: 'GET' });
    return r.ok;
  } catch (e) {
    return false;
  }
}

test.describe('Organization Treasury E2E', () => {
  test('deposit and withdraw via API', async ({ request }) => {
    const uiUrl = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:3000';
    const apiUrl = process.env.PLAYWRIGHT_API_URL || 'http://127.0.0.1:3001';

    const uiOk = await isReachable(uiUrl + '/');
    if (!uiOk) test.skip(true, `UI not reachable at ${uiUrl}`);

    const apiOk = await isReachable(apiUrl + '/plugins/search');
    if (!apiOk) test.skip(true, `Master server not reachable at ${apiUrl}`);

    // Create organization via API
    const createRes = await request.post(`${apiUrl}/v1/orgs`, {
      data: { name: 'T-E2E-Org', tag: 'TE2' },
    });
    expect(createRes.ok()).toBeTruthy();
    const created = await createRes.json();
    const id = created.id;
    expect(id).toBeTruthy();

    // Deposit 1000
    const depRes = await request.post(`${apiUrl}/v1/orgs/${id}/deposit`, { data: { amount: 1000 } });
    expect(depRes.ok()).toBeTruthy();
    const depJson = await depRes.json();
    expect(depJson.balance).toBe(1000);

    // Withdraw 500
    const wRes = await request.post(`${apiUrl}/v1/orgs/${id}/withdraw`, { data: { amount: 500 } });
    expect(wRes.ok()).toBeTruthy();
    const wJson = await wRes.json();
    expect(wJson.balance).toBe(500);

    // Verify via GET
    const gRes = await request.get(`${apiUrl}/v1/orgs/${id}`);
    expect(gRes.ok()).toBeTruthy();
    const gJson = await gRes.json();
    expect(gJson.treasury_balance).toBe(500);
  });
});
