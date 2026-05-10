import { getAdminToken } from './auth';

// Use Vite proxy (relative URLs) in dev, or explicit absolute URL in production.
const API_BASE = import.meta.env.VITE_API_URL ?? '';

export interface ApiError {
  code: string;
  message: string;
}

export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: ApiError;
}

export async function apiFetch<T>(path: string, options?: RequestInit): Promise<ApiResponse<T>> {
  const token = getAdminToken();
  const url = `${API_BASE}${path}`;
  const response = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { 'x-admin-token': token } : {}),
      ...options?.headers,
    },
  });

  if (!response.ok) {
    return {
      ok: false,
      error: {
        code: 'HTTP_ERROR',
        message: `HTTP ${response.status}`,
      },
    };
  }

  const data = await response.json();
  return data as ApiResponse<T>;
}
