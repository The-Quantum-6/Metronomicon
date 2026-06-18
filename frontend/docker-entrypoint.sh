#!/bin/sh
set -eu

# Runs automatically via the nginx image's /docker-entrypoint.d/ mechanism
# before nginx starts. Regenerates runtime config from the environment so the
# frontend can target a different backend per environment without rebuilding.
: "${API_BASE_URL:=/api}"

CONFIG_PATH="/usr/share/nginx/html/config.js"

cat > "$CONFIG_PATH" <<EOF
window.__APP_CONFIG__ = {
  apiBaseUrl: "${API_BASE_URL}",
};
EOF

echo "app-config: wrote $CONFIG_PATH with apiBaseUrl=${API_BASE_URL}"
