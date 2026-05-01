const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
  };
}

export async function apiFetch<T>(path: string, options?: RequestInit): Promise<ApiResponse<T>> {
  const url = `${API_BASE}${path}`;
  const response = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
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
