<<<<<<< Updated upstream
import { useState, useEffect, useCallback } from 'react'
import { login as apiLogin, refreshToken as apiRefreshToken, AuthSession } from '../api/auth'

export type License = 'Free' | 'Pro' | 'Enterprise'

const STORAGE_KEY = 'verseguy_session_v1'

function saveSession(session: AuthSession | null) {
  if (!session) {
    localStorage.removeItem(STORAGE_KEY)
    return
  }
  localStorage.setItem(STORAGE_KEY, JSON.stringify(session))
}

function loadSession(): AuthSession | null {
  const raw = localStorage.getItem(STORAGE_KEY)
  if (!raw) return null
  try {
    return JSON.parse(raw) as AuthSession
  } catch {
    return null
  }
}

export function useAuth() {
  const [session, setSession] = useState<AuthSession | null>(() => loadSession())
  const [loading, setLoading] = useState(false)

  const login = useCallback(async (username: string, password: string) => {
    setLoading(true)
    try {
      const s = await apiLogin(username, password)
      setSession(s)
      saveSession(s)
      setLoading(false)
      return s
    } catch (err) {
      setLoading(false)
      throw err
    }
  }, [])

  const logout = useCallback(() => {
    setSession(null)
    saveSession(null)
  }, [])

  const refreshIfNeeded = useCallback(async () => {
    if (!session?.refresh_token) return
    // If expires_at is set and is in the future, skip refresh
    const now = Date.now()
    if (session.expires_at && session.expires_at - now > 30_000) return

    try {
      const s = await apiRefreshToken(session.refresh_token)
      setSession(s)
      saveSession(s)
    } catch {
      // Refresh failed — logout to force re-login
      logout()
    }
  }, [session, logout])

  useEffect(() => {
    // Periodic refresh (every minute)
    const id = setInterval(() => {
      void refreshIfNeeded()
    }, 60_000)
    return () => clearInterval(id)
  }, [refreshIfNeeded])

  const license: License = (session?.license as License) || 'Free'

  return {
    session,
    license,
    loading,
    login,
    logout,
    refreshIfNeeded
  }
}

export default useAuth
=======
import { useState } from 'react'

export type LicenseKind = 'Community' | 'Pro' | 'Enterprise'

export function useAuth() {
  // Minimal stub for build & e2e; in real app this will connect to auth context
  const [license] = useState<LicenseKind>('Pro')
  return { license }
}
>>>>>>> Stashed changes
