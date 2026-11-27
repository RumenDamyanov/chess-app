import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import '@styles/main.scss';
import { BackendProvider } from './providers';
import App from './App.tsx';

const root = document.getElementById('root');
if (!root) {
  throw new Error('The window root is missing.');
}

createRoot(root).render(
  <StrictMode>
    <BackendProvider>
      <App />
    </BackendProvider>
  </StrictMode>
);
