import React, { useState } from 'react';
import { setAdminToken } from '../api/auth';
import { getConfig } from '../api/admin';

const Login: React.FC<{ onLogin: () => void }> = ({ onLogin }) => {
  const [token, setToken] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!token.trim()) {
      setError('Please enter your admin token');
      return;
    }

    setLoading(true);
    setError('');

    // Set token temporarily for validation
    setAdminToken(token.trim());

    try {
      // Try to call a protected endpoint to validate the token
      await getConfig();
      // If successful, token is valid
      onLogin();
    } catch {
      // Token is invalid
      setAdminToken(null);
      setError('Invalid admin token. Please check your token and try again.');
    } finally {
      setLoading(false);
    }
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
            disabled={loading}
            style={{ width: '100%', padding: '0.5rem', fontSize: '1rem' }}
          />
        </div>
        {error ? <p role="alert" style={{ color: 'red' }}>{error}</p> : null}
        <button type="submit" disabled={loading} style={{ padding: '0.5rem 1rem', fontSize: '1rem' }}>
          {loading ? 'Verifying...' : 'Connect'}
        </button>
      </form>
    </div>
  );
};

export default Login;
