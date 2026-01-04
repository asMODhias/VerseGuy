import React from 'react'
import useAuth, { License } from '../hooks/useAuth'

const levels: Record<License, number> = {
  Free: 0,
  Pro: 1,
  Enterprise: 2
}

type Props = {
  required: License
  children: React.ReactNode
  renderUpgrade?: () => React.ReactNode
}

export default function LicenseGuard({ required, children, renderUpgrade }: Props) {
  const { license } = useAuth()

  if (levels[license] >= levels[required]) {
    return <>{children}</>
  }

  if (renderUpgrade) return <>{renderUpgrade()}</>

  return (
    <div style={{ padding: 12, border: '1px solid rgba(0,217,255,0.12)', borderRadius: 8 }}>
      <p style={{ color: 'var(--muted)', marginBottom: 8 }}>This feature requires <strong>{required}</strong> license.</p>
      <button style={{ background: 'var(--accent)', color: '#00121a', border: 'none', padding: '6px 10px', borderRadius: 6 }}>Upgrade to {required}</button>
    </div>
  )
}
