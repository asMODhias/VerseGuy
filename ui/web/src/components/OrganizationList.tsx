import React, { useState } from 'react'
import { Organization, createOrg } from '../api/organizations'

type Props = {
  orgs: Organization[]
  onCreated?: () => void
}

export default function OrganizationList({ orgs, onCreated }: Props) {
  const [creating, setCreating] = useState(false)
  const [name, setName] = useState('')
  const [tag, setTag] = useState('')
  const [error, setError] = useState<string|undefined>()

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(undefined)
    try {
      setCreating(true)
      await createOrg(name, tag)
      setName('')
      setTag('')
      onCreated?.()
    } catch (err: any) {
      setError(err.message || String(err))
    } finally {
      setCreating(false)
    }
  }

  return (
    <div className="organization-list" aria-label="organization-list">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h3>Organizations</h3>
        <form onSubmit={submit} style={{ display: 'flex', gap: 8 }}>
          <input placeholder="Name" value={name} onChange={e => setName(e.target.value)} />
          <input placeholder="Tag" value={tag} onChange={e => setTag(e.target.value)} />
          <button type="submit" disabled={creating || !name || !tag} style={{ background: 'var(--accent)', color: '#00121a', border: 'none', padding: '6px 10px', borderRadius: 6 }}>
            {creating ? 'Creating…' : 'Create'}
          </button>
        </form>
      </div>

      {error ? <p style={{ color: 'var(--danger)' }}>{error}</p> : null}

      {orgs.length === 0 ? (
        <p style={{ color: 'var(--muted)' }}>No organizations yet.</p>
      ) : (
        <table style={{ width: '100%', marginTop: 12, borderCollapse: 'collapse' }}>
          <thead>
            <tr>
              <th style={{ textAlign: 'left', padding: 8 }}>Name</th>
              <th style={{ textAlign: 'left', padding: 8 }}>Tag</th>
              <th style={{ textAlign: 'left', padding: 8 }}>Members</th>
            </tr>
          </thead>
          <tbody>
            {orgs.map(o => (
              <tr key={o.id}>
                <td style={{ padding: 8 }}>{o.name}</td>
                <td style={{ padding: 8 }}>{o.tag}</td>
                <td style={{ padding: 8 }}>{o.member_count}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  )
}