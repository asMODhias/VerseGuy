import React from 'react'
import { BrowserRouter, Routes, Route, Link } from 'react-router-dom'
import OrganizationTab from './tabs/OrganizationTab'

export default function App() {
  return (
    <BrowserRouter>
      <div className="app-shell">
        <nav className="topbar">
          <Link to="/">Dashboard</Link>
          <Link to="/organization">Organization</Link>
        </nav>

        <main>
          <Routes>
            <Route path="/" element={<div>Welcome to VerseguY</div>} />
            <Route path="/organization" element={<OrganizationTab />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  )
}
