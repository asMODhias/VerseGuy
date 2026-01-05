import React from 'react'
import { render, screen } from '@testing-library/react'
import LicenseGuard from './LicenseGuard'
import '@testing-library/jest-dom'

describe('LicenseGuard', () => {
  beforeEach(() => localStorage.clear())

  it('shows upgrade prompt for Free license', () => {
    render(
      <LicenseGuard required="Pro">
        <div>pro-content</div>
      </LicenseGuard>
    )

    expect(screen.queryByText('pro-content')).toBeNull()
    expect(screen.getByText(/This feature requires/)).toBeInTheDocument()
  })

  it('renders children for Pro license', () => {
    localStorage.setItem('verseguy_session_v1', JSON.stringify({ license: 'Pro' }))
    render(
      <LicenseGuard required="Pro">
        <div>pro-content</div>
      </LicenseGuard>
    )

    expect(screen.getByText('pro-content')).toBeInTheDocument()
  })
})