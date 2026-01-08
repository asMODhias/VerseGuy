import { test, expect } from '@playwright/test';

async function isReachable(url: string) {
  try {
    const r = await fetch(url, { method: 'GET' });
    return r.ok;
  } catch (e) {
    return false;
  }
}

test.describe('Organization Members E2E', () => {
  test('add member via API and verify in org', async ({ request }) => {
    const uiUrl = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:3000';
    const apiUrl = process.env.PLAYWRIGHT_API_URL || 'http://127.0.0.1:3001';

    const uiOk = await isReachable(uiUrl + '/');
    if (!uiOk) test.skip(true, `UI not reachable at ${uiUrl}`);

    const apiOk = await isReachable(apiUrl + '/plugins/search');
    if (!apiOk) test.skip(true, `Master server not reachable at ${apiUrl}`);

    // Create organization
    const createRes = await request.post(`${apiUrl}/v1/orgs`, { data: { name: 'M-E2E-Org', tag: 'ME2' } });
    expect(createRes.ok()).toBeTruthy();
    const created = await createRes.json();
    const id = created.id;
    expect(id).toBeTruthy();

    // Add a member
    const memberRes = await request.post(`${apiUrl}/v1/orgs/${id}/members`, {
      data: { user_id: 'e2e-user-1', rank_id: null },
    });
    expect(memberRes.ok()).toBeTruthy();

    // Fetch org and assert member present
    const gRes = await request.get(`${apiUrl}/v1/orgs/${id}`);
    expect(gRes.ok()).toBeTruthy();
    const gJson = await gRes.json();
    const members = gJson.members || [];
    expect(Array.isArray(members)).toBeTruthy();
    expect(members.some((m: any) => m.user_id === 'e2e-user-1')).toBeTruthy();
  });
});
