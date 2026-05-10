// In-memory admin token -- session only, never persisted.
// Set via setAdminToken(), read by apiFetch().

let _token: string | null = null;

export function setAdminToken(token: string | null): void {
  _token = token;
}

export function getAdminToken(): string | null {
  return _token;
}
