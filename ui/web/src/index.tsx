import React from 'react';
import { createRoot } from 'react-dom/client';

import OrganizationTab from './tabs/OrganizationTab'

const App = () => (
  <div>
    <OrganizationTab />
  </div>
)

const root = document.createElement('div');
root.id = 'root';
document.body.appendChild(root);
createRoot(root).render(<App />);
