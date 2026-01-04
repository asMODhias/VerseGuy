export type AuthSession = {
  access_token: string
  refresh_token?: string
  expires_at?: number // unix epoch ms
  user_id?: string
  license?: 'Free' | 'Pro' | 'Enterprise'
}

const BASE = (import.meta.env.VITE_MASTER_SERVER_URL as string) || 'http://localhost:3000'

export async function login(username: string, password: string): Promise<AuthSession> {
  const res = await fetch(`${BASE}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password })
  })

  if (!res.ok) {
    throw new Error(`login failed: ${res.status}`)
  }

  const body = await res.json()
  return normalize(body)
}

export async function refreshToken(refreshToken: string): Promise<AuthSession> {
  const res = await fetch(`${BASE}/auth/refresh`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ refresh_token: refreshToken })
  })

  if (!res.ok) {
    throw new Error(`refresh failed: ${res.status}`)
  }

  const body = await res.json()
  return normalize(body)
}

export async function validateToken(accessToken: string): Promise<AuthSession> {
  const res = await fetch(`${BASE}/auth/validate`, {
    method: 'GET',
    headers: { Authorization: `Bearer ${accessToken}` }
  })

  if (!res.ok) {
    throw new Error(`validate failed: ${res.status}`)
  }

  const body = await res.json()
  return normalize(body)
}

function normalize(body: any): AuthSession {
  // Normalize various server responses into AuthSession
  return {
    access_token: body.access_token || body.token || '',
    refresh_token: body.refresh_token,
    expires_at: body.expires_at ? Number(body.expires_at) : undefined,
    user_id: body.user_id || body.sub,
    license: body.license
  }
}
