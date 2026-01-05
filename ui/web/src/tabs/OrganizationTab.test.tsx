import React from 'react'
import { render, screen } from '@testing-library/react'
import OrganizationTab from './OrganizationTab'
import '@testing-library/jest-dom'

describe('OrganizationTab', () => {
  it('shows upgrade prompt for Free license', () => {
    render(<OrganizationTab />)
    expect(screen.getByText(/Unlock more features:/)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Upgrade to Pro/i })).toBeInTheDocument()
  })
})
