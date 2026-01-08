import { test, expect } from '@playwright/test';

async function isReachable(url: string) {
  try {
    const r = await fetch(url, { method: 'GET' });
    return r.ok;
  } catch (e) {
    return false;
  }
}

test.describe('Fleet E2E', () => {
  test('create fleet, add ship via API and verify', async ({ request }) => {
    const uiUrl = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:3000';
    const apiUrl = process.env.PLAYWRIGHT_API_URL || 'http://127.0.0.1:3001';

    const uiOk = await isReachable(uiUrl + '/');
    if (!uiOk) test.skip(true, `UI not reachable at ${uiUrl}`);

    const apiOk = await isReachable(apiUrl + '/plugins/search');
    if (!apiOk) test.skip(true, `Master server not reachable at ${apiUrl}`);

    // Create fleet
    const createRes = await request.post(`${apiUrl}/v1/fleets`, { data: { name: 'F-E2E-Fleet' } });
    expect(createRes.ok()).toBeTruthy();
    const created = await createRes.json();
    const id = created.id;
    expect(id).toBeTruthy();

    // Add a ship
    const shipRes = await request.post(`${apiUrl}/v1/fleets/${id}/ships`, {
      data: { ship_type_id: 'st-e2e', name: 'E2E-Ship-1' },
    });
    expect(shipRes.ok()).toBeTruthy();

    // Fetch fleet and assert ship present
    const gRes = await request.get(`${apiUrl}/v1/fleets/${id}`);
    expect(gRes.ok()).toBeTruthy();
    const gJson = await gRes.json();
    const ships = gJson.ships || [];
    expect(Array.isArray(ships)).toBeTruthy();
    expect(ships.some((s: any) => s.ship_type_id === 'st-e2e' && s.name === 'E2E-Ship-1')).toBeTruthy();
  });
});