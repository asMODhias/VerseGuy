import React from 'react'
import { render, screen } from '@testing-library/react'
import RankManagement from './RankManagement'
import '@testing-library/jest-dom'

describe('RankManagement', () => {
  it('renders empty state', () => {
    render(<RankManagement ranks={[]} />)
    expect(screen.getByText('No ranks defined.')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /add rank/i })).toBeDisabled()
  })

  it('renders ranks list', () => {
    const ranks = [
      { id: 'r1', name: 'Officer', level: 2 },
      { id: 'r2', name: 'Member' }
    ]

    render(<RankManagement ranks={ranks} />)

    expect(screen.getByText('Officer')).toBeInTheDocument()
    expect(screen.getByText(/Level 2/)).toBeInTheDocument()
    expect(screen.getByText('Member')).toBeInTheDocument()
  })
})