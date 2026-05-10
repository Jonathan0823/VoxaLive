import { BrowserRouter, Routes, Route, Link } from 'react-router-dom';
import { useState } from 'react';
import Dashboard from './pages/Dashboard';
import Providers from './pages/Providers';
import Voice from './pages/Voice';
import LiveInputs from './pages/LiveInputs';
import TestConsole from './pages/TestConsole';
import Secrets from './pages/Secrets';
import Login from './pages/Login';
import { getAdminToken } from './api/auth';

function App() {
  const [authenticated, setAuthenticated] = useState(() => !!getAdminToken());

  if (!authenticated) {
    return <Login onLogin={() => setAuthenticated(true)} />;
  }

  return (
    <BrowserRouter>
      <div style={{ display: 'flex' }}>
        <nav style={{ width: '200px', padding: '1rem' }}>
          <h2>VoxaLive Admin</h2>
          <ul style={{ listStyle: 'none', padding: 0 }}>
            <li><Link to="/">Dashboard</Link></li>
            <li><Link to="/providers">Providers</Link></li>
            <li><Link to="/voice">Voice / TTS</Link></li>
            <li><Link to="/live-inputs">Live Inputs</Link></li>
            <li><Link to="/secrets">Secrets</Link></li>
            <li><Link to="/test">Test Console</Link></li>
          </ul>
        </nav>
        <main style={{ flex: 1, padding: '1rem' }}>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/providers" element={<Providers />} />
            <Route path="/voice" element={<Voice />} />
            <Route path="/live-inputs" element={<LiveInputs />} />
            <Route path="/secrets" element={<Secrets />} />
            <Route path="/test" element={<TestConsole />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default App;
