FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# runtime dependencies may be added here

COPY retention_runner /usr/local/bin/retention_runner
RUN chmod +x /usr/local/bin/retention_runner

ENTRYPOINT ["/usr/local/bin/retention_runner"]
