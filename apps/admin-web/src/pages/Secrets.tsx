import React, { useState, useEffect } from 'react';
import { getSecretStatus, putSecrets, type SecretStatusData } from '../api/admin';

type SecretEntry = {
  key: string;
  label: string;
  value: string;
  loading: boolean;
  success: boolean;
  error: string | null;
};

const SECRET_FIELDS: { key: string; label: string }[] = [
  { key: 'gemini_api_key', label: 'Gemini API Key' },
  { key: 'openrouter_api_key', label: 'OpenRouter API Key' },
  { key: 'vts_auth_token', label: 'VTube Studio Auth Token' },
  { key: 'admin_token', label: 'Admin Token' },
];

const Secrets: React.FC = () => {
  const [status, setStatus] = useState<SecretStatusData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [entries, setEntries] = useState<Record<string, SecretEntry>>(() =>
    Object.fromEntries(
      SECRET_FIELDS.map(({ key, label }) => [
        key,
        { key, label, value: '', loading: false, success: false, error: null },
      ])
    )
  );

  useEffect(() => {
    loadStatus();
  }, []);

  const loadStatus = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await getSecretStatus();
      setStatus(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load secret status');
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (key: string, val: string) => {
    setEntries(prev => ({
      ...prev,
      [key]: { ...prev[key], value: val, success: false, error: null },
    }));
  };

  const handleSubmit = async (key: string) => {
    const entry = entries[key];
    if (!entry) return;

    setEntries(prev => ({
      ...prev,
      [key]: { ...prev[key], loading: true, success: false, error: null },
    }));

    // Build secrets payload: empty string = clear, non-empty = set
    const secrets: Record<string, string> = {};
    for (const field of SECRET_FIELDS) {
      secrets[field.key] = field.key === key ? entry.value : '';
    }

    try {
      await putSecrets(secrets);
      setEntries(prev => ({
        ...prev,
        [key]: { ...prev[key], loading: false, success: true, error: null },
      }));
      await loadStatus();
    } catch (err) {
      setEntries(prev => ({
        ...prev,
        [key]: {
          ...prev[key],
          loading: false,
          success: false,
          error: err instanceof Error ? err.message : 'Update failed',
        },
      }));
    }
  };

  const handleClear = async (key: string) => {
    setEntries(prev => ({
      ...prev,
      [key]: { ...prev[key], value: '', loading: false, success: false, error: null },
    }));
    const secrets: Record<string, string> = {};
    for (const field of SECRET_FIELDS) {
      secrets[field.key] = field.key === key ? '' : '';
    }
    try {
      await putSecrets(secrets);
      await loadStatus();
    } catch (err) {
      setEntries(prev => ({
        ...prev,
        [key]: {
          ...prev[key],
          loading: false,
          success: false,
          error: err instanceof Error ? err.message : 'Clear failed',
        },
      }));
    }
  };

  if (loading) {
    return <p className="p-6">Loading secret status...</p>;
  }

  if (error) {
    return (
      <div className="p-6">
        <p className="text-red-600" role="alert">{error}</p>
        <button onClick={loadStatus} className="mt-2 text-blue-600 underline">Retry</button>
      </div>
    );
  }

  return (
    <div className="p-6 max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold text-gray-900 mb-2">Secrets</h1>
      <p className="text-gray-600 mb-6">
        Manage API keys and tokens. Raw values are never returned after submission.
      </p>

      <div className="space-y-6">
        {SECRET_FIELDS.map(({ key, label }) => {
          const entry = entries[key];
          const isConfigured = status?.secrets[key]?.configured ?? false;

          return (
            <div key={key} className="p-6 bg-white rounded-lg shadow-sm border border-gray-200">
              <div className="flex items-center justify-between mb-3">
                <h2 className="text-base font-medium text-gray-900">{label}</h2>
                <span className={`text-xs px-2 py-1 rounded-full font-medium ${
                  isConfigured
                    ? 'bg-green-100 text-green-700'
                    : 'bg-gray-100 text-gray-500'
                }`}>
                  {isConfigured ? 'configured' : 'not set'}
                </span>
              </div>

              <div className="space-y-3">
                <input
                  type="password"
                  placeholder={isConfigured ? '•••••••••• (leave blank to keep current)' : 'Enter value...'}
                  value={entry.value}
                  onChange={(e) => handleChange(key, e.target.value)}
                  disabled={entry.loading}
                  className="w-full border border-gray-300 rounded-md py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50"
                />

                {entry.error && (
                  <p className="text-red-600 text-sm" role="alert">{entry.error}</p>
                )}
                {entry.success && (
                  <p className="text-green-600 text-sm">Updated successfully.</p>
                )}

                <div className="flex gap-2">
                  <button
                    type="button"
                    onClick={() => handleSubmit(key)}
                    disabled={entry.loading}
                    className="inline-flex justify-center py-1.5 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {entry.loading ? 'Saving...' : 'Save'}
                  </button>
                  {isConfigured && (
                    <button
                      type="button"
                      onClick={() => handleClear(key)}
                      disabled={entry.loading}
                      className="inline-flex justify-center py-1.5 px-4 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
                    >
                      Clear
                    </button>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default Secrets;