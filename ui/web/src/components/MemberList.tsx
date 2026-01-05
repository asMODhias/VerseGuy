import React from 'react'

<<<<<<< Updated upstream
export type Member = {
  id: string
  name: string
  role?: string
}

type Props = {
  members: Member[]
}

export default function MemberList({ members }: Props) {
  return (
    <div className="member-list" aria-label="member-list">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h3>Members</h3>
        <button disabled style={{ background: 'var(--accent)', color: '#00121a', border: 'none', padding: '6px 10px', borderRadius: 6 }}>
          Add member
        </button>
      </div>

      {members.length === 0 ? (
        <p style={{ color: 'var(--muted)' }}>No members yet.</p>
      ) : (
        <table style={{ width: '100%', marginTop: 12, borderCollapse: 'collapse' }}>
          <thead>
            <tr>
              <th style={{ textAlign: 'left', padding: 8 }}>Name</th>
              <th style={{ textAlign: 'left', padding: 8 }}>Role</th>
            </tr>
          </thead>
          <tbody>
            {members.map(m => (
              <tr key={m.id}>
                <td style={{ padding: 8 }}>{m.name}</td>
                <td style={{ padding: 8 }}>{m.role ?? 'Member'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
=======
export default function MemberList({ members }: { members: any[] }) {
  return (
    <div>
      <h3>Members</h3>
      <p style={{ color: 'var(--muted)' }}>Member list placeholder ({members.length})</p>
>>>>>>> Stashed changes
    </div>
  )
}
