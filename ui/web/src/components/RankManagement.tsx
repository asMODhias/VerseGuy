import React from 'react'

export type Rank = {
  id: string
  name: string
  level?: number
}

type Props = {
  ranks: Rank[]
}

export default function RankManagement({ ranks }: Props) {
  return (
    <div className="rank-management" aria-label="rank-management">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h3>Ranks</h3>
        <button disabled style={{ background: 'transparent', color: 'var(--accent)', border: '1px solid rgba(0,217,255,0.1)', padding: '6px 10px', borderRadius: 6 }}>
          Add rank
        </button>
      </div>

      {ranks.length === 0 ? (
        <p style={{ color: 'var(--muted)' }}>No ranks defined.</p>
      ) : (
        <ul style={{ paddingLeft: 0, listStyle: 'none', marginTop: 12 }}>
          {ranks.map(r => (
            <li key={r.id} style={{ padding: '6px 0' }}>
              <strong>{r.name}</strong> {r.level ? <span style={{ color: 'var(--muted)' }}>• Level {r.level}</span> : null}
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}
