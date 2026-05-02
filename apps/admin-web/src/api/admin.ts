import { apiFetch } from './client';

export interface ApiError {
  code: string;
  message: string;
}

export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: ApiError;
}

export interface HealthData {
  status: string;
  version: string;
  uptime_sec: number;
  active_llm_provider: string;
  active_tts_provider: string;
}

export interface LlmConfig {
  provider: string;
  model: string;
  temperature: number;
  max_tokens: number;
}

export interface TtsConfig {
  mode: string;
  provider: string;
  model_path?: string;
}

export interface SttConfig {
  device: string;
}

export interface LiveConfig {
  enabled: boolean;
  youtube_video_id: string;
  tiktok_room: string;
}

export interface ServerConfig {
  admin_ui_enabled: boolean;
}

export interface ConfigData {
  llm: LlmConfig;
  tts: TtsConfig;
  stt: SttConfig;
  live: LiveConfig;
  server: ServerConfig;
}

export interface ConfigResponse {
  data: ConfigData;
}

export interface ConfigPatchRequest {
  llm?: LlmConfig;
  tts?: TtsConfig;
  stt?: SttConfig;
  live?: LiveConfig;
}

export interface SecretStatus {
  configured: boolean;
}

export interface SecretStatusData {
  secrets: Record<string, SecretStatus>;
}

const unwrap = <T>(response: ApiResponse<T>): T => {
  if (!response.ok || !response.data) {
    throw new Error(response.error?.message || 'Request failed');
  }

  return response.data;
};

export const getHealth = async (): Promise<HealthData> => {
  const response = await apiFetch<HealthData>('/api/health');
  return unwrap(response);
};

export const getConfig = async (): Promise<ConfigData> => {
  const response = await apiFetch<ConfigResponse>('/api/config');
  return unwrap(response).data;
};

export const patchConfig = async (patch: ConfigPatchRequest): Promise<ConfigData> => {
  const response = await apiFetch<ConfigResponse>('/api/config', {
    method: 'PATCH',
    body: JSON.stringify(patch),
  });
  return unwrap(response).data;
};

export const getSecretStatus = async (): Promise<SecretStatusData> => {
  const response = await apiFetch<SecretStatusData>('/api/secrets/status');
  return unwrap(response);
};

export const putSecrets = async (secrets: Record<string, string>): Promise<string[]> => {
  const response = await apiFetch<{ data: { updated: string[] } }>('/api/secrets', {
    method: 'PUT',
    body: JSON.stringify(secrets),
  });
  return unwrap(response).data.updated;
};
