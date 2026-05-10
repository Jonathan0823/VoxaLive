import React, { useState } from 'react';
import { setAdminToken } from '../api/auth';

const Login: React.FC<{ onLogin: () => void }> = ({ onLogin }) => {
  const [token, setToken] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!token.trim()) {
      setError('Please enter your admin token');
      return;
    }
    setAdminToken(token.trim());
    onLogin();
  };

  return (
    <div style={{ maxWidth: '400px', margin: '4rem auto', padding: '2rem' }}>
      <h1>VoxaLive Admin</h1>
      <p>Enter your admin token to continue.</p>
      <form onSubmit={handleSubmit}>
        <div style={{ marginBottom: '1rem' }}>
          <input
            type="password"
            placeholder="Admin token"
            value={token}
            onChange={(e) => setToken(e.target.value)}
            style={{ width: '100%', padding: '0.5rem', fontSize: '1rem' }}
          />
        </div>
        {error ? <p role="alert" style={{ color: 'red' }}>{error}</p> : null}
        <button type="submit" style={{ padding: '0.5rem 1rem', fontSize: '1rem' }}>
          Connect
        </button>
      </form>
    </div>
  );
};

export default Login;
