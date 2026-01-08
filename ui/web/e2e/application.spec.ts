import { test, expect } from '@playwright/test';

async function isReachable(url: string) {
  try {
    const r = await fetch(url, { method: 'GET' });
    return r.ok;
  } catch (e) {
    return false;
  }
}

test.describe('Application E2E', () => {
  test('create app and fetch via API', async ({ request }) => {
    const uiUrl = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:3000';
    const apiUrl = process.env.PLAYWRIGHT_API_URL || 'http://127.0.0.1:3001';

    const uiOk = await isReachable(uiUrl + '/');
    if (!uiOk) test.skip(true, `UI not reachable at ${uiUrl}`);

    const apiOk = await isReachable(apiUrl + '/plugins/search');
    if (!apiOk) test.skip(true, `Master server not reachable at ${apiUrl}`);

    const createRes = await request.post(`${apiUrl}/v1/apps`, { data: { name: 'App-E2E' } });
    expect(createRes.ok()).toBeTruthy();
    const created = await createRes.json();
    const id = created.id;

    const gRes = await request.get(`${apiUrl}/v1/apps/${id}`);
    expect(gRes.ok()).toBeTruthy();
    const gJson = await gRes.json();
    expect(gJson.name).toBe('App-E2E');

    // Update name
    const updRes = await request.patch(`${apiUrl}/v1/apps/${id}`, { data: { name: 'App-E2E-Updated' } });
    expect(updRes.ok()).toBeTruthy();
    const gRes2 = await request.get(`${apiUrl}/v1/apps/${id}`);
    const gJson2 = await gRes2.json();
    expect(gJson2.name).toBe('App-E2E-Updated');

    // Add metadata and tags
    const metaRes = await request.patch(`${apiUrl}/v1/apps/${id}`, { data: { metadata: { env: 'staging', owner: 'team-a' }, tags: ['beta','internal'] } });
    expect(metaRes.ok()).toBeTruthy();
    const gRes3 = await request.get(`${apiUrl}/v1/apps/${id}`);
    const gJson3 = await gRes3.json();
    expect(gJson3.metadata?.env).toBe('staging');
    expect(gJson3.tags).toEqual(expect.arrayContaining(['beta']));

    // Delete
    const delRes = await request.delete(`${apiUrl}/v1/apps/${id}`);
    expect(delRes.ok()).toBeTruthy();
    const gRes3 = await request.get(`${apiUrl}/v1/apps/${id}`);
    const gJson3 = await gRes3.json();
    expect(gJson3).toBeNull();
  });
});