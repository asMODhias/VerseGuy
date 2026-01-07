# Telemetry test stack

This directory contains the Docker Compose setup and Collector configuration used by the `telemetry-e2e` integration test.

Key points:

- We run an OpenTelemetry Collector and Jaeger (all-in-one) in Docker Compose for CI/local tests.
- The Collector is configured to receive OTLP (gRPC at 4317 and HTTP at 4318) and export traces to Jaeger using the Zipkin HTTP endpoint at `http://jaeger:9411/api/v2/spans`.
- Jaeger is started with its Zipkin listener enabled so the Collector can push Zipkin-format spans directly to Jaeger on container-internal port `9411`.

Test behavior:

- The `telemetry-e2e` test sends spans via OTLP (HTTP or gRPC depending on env) to the Collector and asserts that Jaeger contains the trace for the test_id.
- The test now asserts against Jaeger as the canonical sink. Previously we had a Zipkin fallback for debugging; we removed it to avoid ambiguity.

CI notes:

- When running the test in CI, make sure Docker Compose starts the `jaeger` and `otel-collector` services from this folder first. The CI job should wait until the Collector logs show "Everything is ready." and Jaeger logs show "Listening for Zipkin HTTP traffic".
- Diagnostics: on timeout the test writes `target/telemetry_e2e_timeout_diag.json` so CI jobs can upload this artifact for debugging.

If you need me to update the CI workflow to use this compose stack and run the test in CI, say so and I'll make the change.