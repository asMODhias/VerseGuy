import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import useAuth from './useAuth'

const mockLoginResponse = {
  access_token: 'a-token',
  refresh_token: 'r-token',
  expires_at: Date.now() + 60 * 60 * 1000,
  user_id: 'u1',
  license: 'Pro'
}

describe('useAuth', () => {
  beforeEach(() => {
    localStorage.clear()
    vi.restoreAllMocks()
  })

  it('login stores session and exposes license', async () => {
    vi.stubGlobal('fetch', vi.fn(async (url, opts) => {
      if ((url as string).endsWith('/auth/login')) {
        return { ok: true, json: async () => mockLoginResponse }
      }
      return { ok: false, status: 500 }
    }))

    const { result } = renderHook(() => useAuth())

    await act(async () => {
      const s = await result.current.login('user', 'pass')
      expect(s.access_token).toBe('a-token')
    })

    expect(result.current.license).toBe('Pro')
    // session saved in localStorage
    expect(localStorage.getItem('verseguy_session_v1')).toBeTruthy()
  })

  it('logout clears session', async () => {
    // set initial
    localStorage.setItem('verseguy_session_v1', JSON.stringify(mockLoginResponse))
    const { result } = renderHook(() => useAuth())

    act(() => result.current.logout())
    expect(result.current.session).toBeNull()
    expect(localStorage.getItem('verseguy_session_v1')).toBeNull()
  })
})
