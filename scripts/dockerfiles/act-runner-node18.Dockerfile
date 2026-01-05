FROM ubuntu:20.04

# Install Node.js 18 (to provide crypto.randomUUID and modern action support)
USER root
RUN set -eux; \
    NODE_VERSION="18.20.0"; \
    # Force-install Node binary to /usr/local (overwrite older node versions)
    rm -f /usr/bin/node /usr/bin/npm /usr/local/bin/node /usr/local/bin/npm || true; \
    if command -v curl >/dev/null 2>&1; then \
        curl -fsSLO "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz"; \
    else \
        apt-get update || true; apt-get install -y curl xz-utils tar ca-certificates --no-install-recommends || true; \
        curl -fsSLO "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz"; \
    fi; \
    tar -xJf "node-v${NODE_VERSION}-linux-x64.tar.xz" -C /usr/local --strip-components=1; \
    rm -f "node-v${NODE_VERSION}-linux-x64.tar.xz"; \
    # Verify
    /usr/local/bin/node --version; /usr/local/bin/npm --version; \
    rm -rf /tmp/* /var/tmp/* || true

# Verify node
RUN which node || true && node --version || true && npm --version || true
