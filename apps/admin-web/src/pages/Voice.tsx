import React, { useState, useEffect, useRef } from 'react';
import { getConfig, patchConfig, type ConfigData, type TtsConfig } from '../api/admin';

interface VoiceState {
  config: ConfigData | null;
  loading: boolean;
  error: string | null;
  saveLoading: boolean;
  saveError: string | null;
  saveSuccess: boolean;
}

const Voice: React.FC = () => {
  const mounted = useRef(true);
  const [state, setState] = useState<VoiceState>({
    config: null,
    loading: true,
    error: null,
    saveLoading: false,
    saveError: null,
    saveSuccess: false,
  });
  const [formData, setFormData] = useState<Partial<TtsConfig>>({});

  useEffect(() => {
    mounted.current = true;

    const loadConfig = async () => {
      try {
        const config = await getConfig();
        if (!mounted.current) return;
        setState(prev => ({ ...prev, config, loading: false, error: null }));
        setFormData(config.tts);
      } catch (err) {
        if (!mounted.current) return;
        setState(prev => ({
          ...prev,
          loading: false,
          error: err instanceof Error ? err.message : 'Failed to load TTS settings',
        }));
      }
    };

    void loadConfig();

    return () => {
      mounted.current = false;
    };
  }, []);

  const handleProviderChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setFormData(prev => ({ ...prev, provider: e.target.value }));
  };

  const handleModeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setFormData(prev => ({ ...prev, mode: e.target.value }));
  };

  const handleModelPathChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData(prev => ({ ...prev, model_path: e.target.value || undefined }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setState(prev => ({ ...prev, saveLoading: true, saveError: null, saveSuccess: false }));

    try {
      await patchConfig({ tts: formData as TtsConfig });
      const updatedConfig = await getConfig(); // Refresh config per requirements
      if (!mounted.current) return;
      setState(prev => ({
        ...prev,
        config: updatedConfig,
        saveLoading: false,
        saveSuccess: true,
      }));
      setFormData(updatedConfig.tts);
      // Clear success message after 3 seconds
      setTimeout(() => {
        if (mounted.current) {
          setState(prev => ({ ...prev, saveSuccess: false }));
        }
      }, 3000);
    } catch (err) {
      if (!mounted.current) return;
      setState(prev => ({
        ...prev,
        saveLoading: false,
        saveError: err instanceof Error ? err.message : 'Failed to save TTS settings',
      }));
    }
  };

  if (state.loading) {
    return (
      <section>
        <h1>Voice / TTS Settings</h1>
        <p>Loading TTS settings…</p>
      </section>
    );
  }

  if (state.error) {
    return (
      <section>
        <h1>Voice / TTS Settings</h1>
        <p role="alert" style={{ color: 'red' }}>{state.error}</p>
      </section>
    );
  }

  return (
    <section>
      <h1>Voice / TTS Settings</h1>
      <p>Configure text-to-speech provider and settings</p>

      {state.saveError ? (
        <p role="alert" style={{ color: 'red' }}>{state.saveError}</p>
      ) : null}
      {state.saveSuccess ? (
        <p style={{ color: 'green' }}>Settings saved successfully!</p>
      ) : null}

      <form onSubmit={handleSubmit} style={{ marginTop: '1rem', display: 'flex', flexDirection: 'column', gap: '1rem', maxWidth: '400px' }}>
        <div>
          <label htmlFor="tts-provider" style={{ display: 'block', marginBottom: '0.5rem' }}>TTS Provider</label>
          <select
            id="tts-provider"
            value={formData.provider || ''}
            onChange={handleProviderChange}
            style={{ padding: '0.5rem', width: '100%' }}
            required
          >
            <option value="piper">Piper</option>
            <option value="qwen">Qwen</option>
          </select>
        </div>

        <div>
          <label htmlFor="tts-mode" style={{ display: 'block', marginBottom: '0.5rem' }}>TTS Mode</label>
          <select
            id="tts-mode"
            value={formData.mode || ''}
            onChange={handleModeChange}
            style={{ padding: '0.5rem', width: '100%' }}
            required
          >
            <option value="cpu">CPU</option>
            <option value="gpu">GPU</option>
            <option value="auto">Auto</option>
          </select>
        </div>

        <div>
          <label htmlFor="tts-model-path" style={{ display: 'block', marginBottom: '0.5rem' }}>Model Path (Optional)</label>
          <input
            id="tts-model-path"
            type="text"
            value={formData.model_path || ''}
            onChange={handleModelPathChange}
            placeholder="./voices/default.onnx"
            style={{ padding: '0.5rem', width: '100%' }}
          />
        </div>

        <button
          type="submit"
          disabled={state.saveLoading}
          style={{
            padding: '0.75rem',
            backgroundColor: state.saveLoading ? '#ccc' : '#007bff',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: state.saveLoading ? 'not-allowed' : 'pointer',
            width: '100%',
          }}
        >
          {state.saveLoading ? 'Saving…' : 'Save Settings'}
        </button>
      </form>
    </section>
  );
};

export default Voice;
