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

export interface ConfigData {
  llm: {
    provider: string;
    model: string;
    temperature: number;
    max_tokens: number;
  };
  tts: {
    mode: string;
    provider: string;
    model_path: string;
  };
  stt: {
    device: string;
  };
  live: {
    youtube_video_id: string;
    tiktok_room: string;
    enabled: boolean;
  };
  server: {
    admin_ui_enabled: boolean;
  };
}

export interface SecretStatusData {
  secrets: Record<string, { configured: boolean }>;
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
  const response = await apiFetch<ConfigData>('/api/config');
  return unwrap(response);
};

export const getSecretStatus = async (): Promise<SecretStatusData> => {
  const response = await apiFetch<SecretStatusData>('/api/secrets/status');
  return unwrap(response);
};
