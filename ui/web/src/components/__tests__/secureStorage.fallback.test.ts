import { vi, describe, it, expect, beforeEach } from 'vitest'
import * as ss from '../../api/secureStorage'

describe('secureStorage fallback', () => {
  beforeEach(() => {
    // Ensure no native host
    ;(window as any).chrome = undefined
    localStorage.clear()
  })

  it('falls back to localStorage', async () => {
    await ss.secureSet('k1', 'v1')
    const v = await ss.secureGet('k1')
    expect(v).toBe('v1')

    await ss.secureRemove('k1')
    const v2 = await ss.secureGet('k1')
    expect(v2).toBeNull()
  })
})