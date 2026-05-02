import React, { useState, useEffect } from 'react';
import { getConfig, patchConfig, type LiveConfig } from '../api/admin';

const LiveInputs: React.FC = () => {
  const [liveConfig, setLiveConfig] = useState<LiveConfig>({
    enabled: false,
    youtube_video_id: '',
    tiktok_room: '',
  });
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [saveSuccess, setSaveSuccess] = useState(false);

  useEffect(() => {
    let active = true;

    const loadConfig = async () => {
      try {
        setLoading(true);
        setError(null);
        const configData = await getConfig();
        if (!active) return;
        setLiveConfig(configData.live);
      } catch (err) {
        if (!active) return;
        setError(err instanceof Error ? err.message : 'Failed to load live input settings');
      } finally {
        if (active) setLoading(false);
      }
    };

    void loadConfig();

    return () => {
      active = false;
    };
  }, []);

  const handleSave = async () => {
    // Validate required fields if live inputs are enabled
    if (liveConfig.enabled) {
      if (!liveConfig.youtube_video_id.trim()) {
        setSaveError('YouTube Video ID is required when live inputs are enabled');
        return;
      }
      if (!liveConfig.tiktok_room.trim()) {
        setSaveError('TikTok Room is required when live inputs are enabled');
        return;
      }
    }

    setSaving(true);
    setSaveError(null);
    setSaveSuccess(false);

    try {
      await patchConfig({ live: liveConfig });
      // Refresh config from backend to update displayed state
      const updatedConfig = await getConfig();
      setLiveConfig(updatedConfig.live);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (err) {
      setSaveError(err instanceof Error ? err.message : 'Failed to save settings');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div>
        <h1>Live Inputs</h1>
        <p>Loading live input settings...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div>
        <h1>Live Inputs</h1>
        <p role="alert" style={{ color: 'red' }}>Error: {error}</p>
      </div>
    );
  }

  return (
    <div>
      <h1>Live Inputs</h1>
      <p>Configure YouTube and TikTok live comment inputs</p>

      <div style={{ marginTop: '1rem', display: 'flex', flexDirection: 'column', gap: '1rem', maxWidth: '500px' }}>
        {/* Enabled Toggle */}
        <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <input
            type="checkbox"
            checked={liveConfig.enabled}
            onChange={(e) => setLiveConfig(prev => ({ ...prev, enabled: e.target.checked }))}
          />
          <span>Enable Live Inputs</span>
        </label>

        {/* Editable fields shown only when enabled */}
        {liveConfig.enabled && (
          <>
            {/* YouTube Video ID */}
            <div>
              <label style={{ display: 'block', marginBottom: '0.25rem' }}>
                YouTube Video ID
              </label>
              <input
                type="text"
                value={liveConfig.youtube_video_id}
                onChange={(e) => setLiveConfig(prev => ({ ...prev, youtube_video_id: e.target.value }))}
                placeholder="Enter YouTube Video ID (e.g., dQw4w9WgXcQ)"
                style={{ width: '100%', padding: '0.5rem', boxSizing: 'border-box' }}
                required
              />
            </div>

            {/* TikTok Room */}
            <div>
              <label style={{ display: 'block', marginBottom: '0.25rem' }}>
                TikTok Room
              </label>
              <input
                type="text"
                value={liveConfig.tiktok_room}
                onChange={(e) => setLiveConfig(prev => ({ ...prev, tiktok_room: e.target.value }))}
                placeholder="Enter TikTok Room identifier"
                style={{ width: '100%', padding: '0.5rem', boxSizing: 'border-box' }}
              />
            </div>
          </>
        )}

        {/* Save Button */}
        <div>
          <button
            type="button"
            onClick={handleSave}
            disabled={saving}
            style={{
              padding: '0.5rem 1rem',
              backgroundColor: saving ? '#ccc' : '#333',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: saving ? 'not-allowed' : 'pointer',
              fontSize: '1rem',
            }}
          >
            {saving ? 'Saving...' : 'Save Settings'}
          </button>
        </div>

        {/* Save Error */}
        {saveError && (
          <p role="alert" style={{ color: 'red', marginTop: '0.5rem' }}>Error: {saveError}</p>
        )}

        {/* Save Success */}
        {saveSuccess && (
          <p style={{ color: 'green', marginTop: '0.5rem' }}>Settings saved successfully!</p>
        )}
      </div>
    </div>
  );
};

export default LiveInputs;
