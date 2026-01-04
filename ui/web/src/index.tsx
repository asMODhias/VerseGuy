import React from 'react';
import { createRoot } from 'react-dom/client';

const App = () => <div>Hello VerseguY (web)</div>;

const root = document.createElement('div');
root.id = 'root';
document.body.appendChild(root);
createRoot(root).render(<App />);
