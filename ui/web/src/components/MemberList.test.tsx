import React from 'react'
import { render, screen } from '@testing-library/react'
import MemberList from './MemberList'
import '@testing-library/jest-dom'

describe('MemberList', () => {
  it('renders empty state', () => {
    render(<MemberList members={[]} />)
    expect(screen.getByText('No members yet.')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /add member/i })).toBeDisabled()
  })

  it('renders members table', () => {
    const members = [
      { id: '1', name: 'Alice', role: 'Leader' },
      { id: '2', name: 'Bob' }
    ]

    render(<MemberList members={members} />)

    expect(screen.getByText('Alice')).toBeInTheDocument()
    expect(screen.getByText('Leader')).toBeInTheDocument()
    expect(screen.getByText('Bob')).toBeInTheDocument()
  })
})
