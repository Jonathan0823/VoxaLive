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
  service_url: string;
}

export interface SttConfig {
  service_url: string;
}

export interface LiveConfig {
  enabled: boolean;
  youtube_video_id: string;
  tiktok_room: string;
}

export interface ServerConfig {
  vts_endpoint: string;
}

export interface ServerPatchConfig {
  vts_endpoint?: string;
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
  server?: ServerPatchConfig;
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
  const response = await apiFetch<SecretUpdateResponse>('/api/secrets', {
    method: 'PUT',
    body: JSON.stringify(secrets),
  });
  return unwrap(response).updated;
};

/// Alias for internal use.
type SecretUpdateResponse = { updated: string[] };

// ========== Provider Tests ==========

export interface LlmTestRequest {
  message: string;
  provider?: string;
}

export interface LlmTestResponse {
  provider: string;
  model: string;
  latency_ms: number;
  text: string;
}

export interface TtsTestRequest {
  text: string;
  provider?: string;
}

export interface TtsTestResponse {
  provider: string;
  latency_ms: number;
  audio_format: string;
  audio_base64?: string;
}

export interface VtsTestResponse {
  provider: string;
  success: boolean;
  message: string;
  latency_ms?: number;
}

export const testLlm = async (req: LlmTestRequest): Promise<LlmTestResponse> => {
  const response = await apiFetch<LlmTestResponse>('/api/test/llm', {
    method: 'POST',
    body: JSON.stringify(req),
  });
  return unwrap(response);
};

export const testTts = async (req: TtsTestRequest): Promise<TtsTestResponse> => {
  const response = await apiFetch<TtsTestResponse>('/api/test/tts', {
    method: 'POST',
    body: JSON.stringify(req),
  });
  return unwrap(response);
};

export const testVts = async (): Promise<VtsTestResponse> => {
  const response = await apiFetch<VtsTestResponse>('/api/test/vts', {
    method: 'POST',
    body: '{}',
  });
  return unwrap(response);
};
