import React, { useEffect, useState } from 'react';
import { getConfig, getHealth, getSecretStatus, type ConfigData, type HealthData, type SecretStatusData } from '../api/admin';

type DashboardState = {
  health: HealthData | null;
  config: ConfigData | null;
  secrets: SecretStatusData | null;
  loading: boolean;
  error: string | null;
};

const Dashboard: React.FC = () => {
  const [state, setState] = useState<DashboardState>({
    health: null,
    config: null,
    secrets: null,
    loading: true,
    error: null,
  });

  useEffect(() => {
    let active = true;

    const load = async () => {
      try {
        const [health, config, secrets] = await Promise.all([
          getHealth(),
          getConfig(),
          getSecretStatus(),
        ]);

        if (!active) return;

        setState({ health, config, secrets, loading: false, error: null });
      } catch (error) {
        if (!active) return;

        setState({
          health: null,
          config: null,
          secrets: null,
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to load dashboard data',
        });
      }
    };

    void load();

    return () => {
      active = false;
    };
  }, []);

  const configuredSecrets = Object.entries(state.secrets?.secrets ?? {}).filter(([, value]) => value.configured);

  return (
    <section>
      <h1>Dashboard</h1>
      <p>VoxaLive Admin - live backend overview</p>

      {state.loading ? <p>Loading current backend state…</p> : null}
      {state.error ? <p role="alert">{state.error}</p> : null}

      {!state.loading && !state.error && state.health && state.config && state.secrets ? (
        <div>
          <section>
            <h2>Runtime</h2>
            <p>Status: {state.health.status}</p>
            <p>Version: {state.health.version}</p>
            <p>Uptime: {state.health.uptime_sec}s</p>
          </section>

          <section>
            <h2>Providers</h2>
            <p>LLM: {state.health.active_llm_provider}</p>
            <p>TTS: {state.health.active_tts_provider}</p>
            <p>LLM model: {state.config.llm.model}</p>
            <p>TTS model path: {state.config.tts.model_path}</p>
          </section>

          <section>
            <h2>Secrets</h2>
            <p>Configured secrets: {configuredSecrets.length}</p>
            <ul>
              {configuredSecrets.map(([name]) => (
                <li key={name}>{name}</li>
              ))}
            </ul>
          </section>
        </div>
      ) : null}
    </section>
  );
};

export default Dashboard;
