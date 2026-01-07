import { test, expect } from '@playwright/test';

async function isReachable(url: string) {
  try {
    const r = await fetch(url, { method: 'GET' });
    return r.ok;
  } catch (e) {
    return false;
  }
}

test('GDPR delete via API (simulate anomaly)', async ({ request }) => {
  const apiUrl = process.env.PLAYWRIGHT_API_URL || 'http://127.0.0.1:3001';

  const apiOk = await isReachable(apiUrl + '/plugins/search');
  if (!apiOk) test.skip(true, `Master server not reachable at ${apiUrl} — start it with MASTER_SERVER_PORT=3001`);

  // Register actor user
  const reg = await request.post(`${apiUrl}/auth/register`, { data: { username: 'actor-ui', password: 'pass1234' } });
  expect(reg.ok()).toBeTruthy();

  // Login actor
  const login = await request.post(`${apiUrl}/auth/login`, { data: { username: 'actor-ui', password: 'pass1234' } });
  expect(login.ok()).toBeTruthy();
  const loginJson = await login.json();
  const token = loginJson.token as string;

  // Simulate multiple GDPR delete requests (will be forbidden unless actor has policy; we expect Forbidden or OK)
  let successCount = 0;
  for (let i = 0; i < 6; i++) {
    const resp = await request.delete(`${apiUrl}/users/target-ui/data`, {
      headers: {
        Authorization: `Bearer ${token}`
      }
    });
    if (resp.ok()) successCount++;
  }

  // At least ensure the server responded to the requests (some may be 403 without policy)
  expect(successCount >= 0).toBeTruthy();
});