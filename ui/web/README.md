# VerseguY Web UI

This folder contains the React web UI. Use `npm start` to run it locally.

## E2E tests (Playwright)

- Install dev deps: `cd ui/web && npm install`
- Start the web dev server: `npm start` (default: http://127.0.0.1:3000)
- Start the master server for tests on port 3001:
  `MASTER_SERVER_PORT=3001 MASTER_DB_PATH=./tmp/test_db MASTER_LICENSE_SECRET=secret cargo run -p master_server --features run-server`
- Run Playwright tests: `npm run test:e2e`

The Playwright test `e2e/organization.spec.ts` will skip automatically if the UI or API are not reachable.
