import React from 'react'
import { useAuth } from '../hooks/useAuth'
import MemberList from '../components/MemberList'
import RankManagement from '../components/RankManagement'

export default function OrganizationTab() {
  const { license } = useAuth()

  // Data will be loaded from container/plugin in future; UI shows placeholders now
  const sampleMembers = []
  const sampleRanks = []

  return (
    <div className="tab-container">
      <h1 className="text-3xl font-bold mb-6">Organization</h1>

      <section aria-label="members-section">
        <MemberList members={sampleMembers} />
      </section>

      <section aria-label="ranks-section" style={{ marginTop: 24 }}>
        <RankManagement ranks={sampleRanks} />
      </section>

      {/* Pro features guarded by LicenseGuard */}
      <section style={{ marginTop: 24 }}>
        <LicenseGuard required="Pro" renderUpgrade={() => <UpgradeSection features={["Recruitment System", "Organization Analytics"]} requiredLicense="Pro" />}>
          <h2>Recruitment</h2>
          <p>Recruitment workflow (Pro)</p>
        </LicenseGuard>
      </section>

      <section style={{ marginTop: 12 }}>
        <LicenseGuard required="Pro">
          <h2>Analytics</h2>
          <p>Organization analytics (Pro)</p>
        </LicenseGuard>
      </section>

      {license === 'Enterprise' ? (
        <>
          <section style={{ marginTop: 24 }}>
            <h2>Access Control</h2>
            <p>Enterprise RBAC</p>
          </section>
        </>
      ) : null}
    </div>
  )
}

function UpgradeSection({ features, requiredLicense }: { features: string[]; requiredLicense: string }) {
  return (
    <div className="upgrade-prompt" style={{ marginTop: 16, padding: 12, border: '1px solid rgba(0,217,255,0.15)', borderRadius: 8 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <p style={{ color: 'var(--muted)', marginBottom: 8 }}>Unlock more features:</p>
          <ul style={{ color: 'var(--muted)', margin: 0 }}>
            {features.map(f => (
              <li key={f}>• {f}</li>
            ))}
          </ul>
        </div>
        <button style={{ background: 'var(--accent)', color: '#00121a', border: 'none', padding: '8px 12px', borderRadius: 6 }}>Upgrade to {requiredLicense}</button>
      </div>
    </div>
  )
}
