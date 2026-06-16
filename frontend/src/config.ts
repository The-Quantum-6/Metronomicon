// Reads runtime configuration injected via /config.js (window.__APP_CONFIG__).
// Falls back to "/api" so the app still works if config.js is missing.

declare global {
  interface Window {
    __APP_CONFIG__?: {
      apiBaseUrl?: string;
    };
  }
}

const rawBase = window.__APP_CONFIG__?.apiBaseUrl ?? "http://localhost:3000";

// Normalize: strip trailing slash so callers can compose paths predictably.
export const apiBaseUrl = rawBase.replace(/\/+$/, "");

// Build a full API URL from a path (e.g. apiUrl("users") -> "/api/users").
export function apiUrl(path = ""): string {
  const suffix = path.replace(/^\/+/, "");
  return suffix ? `${apiBaseUrl}/${suffix}` : `${apiBaseUrl}/`;
}

export {};
