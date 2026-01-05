# Organization API

Endpoints (master-server):

- GET /v1/orgs
  - Response: { "orgs": [ { id, name, tag, member_count } ] }
- GET /v1/orgs/{id}
  - Response: Organization object or `null` if not found
- POST /v1/orgs
  - Body: { "name": string, "tag": string }
  - Response: Created Organization object (id assigned)

Notes:
- These endpoints are backed by the `plugins-base-organization` service which stores data in `verseguy_storage` (RocksDB).
- IDs are generated using UUIDv4 when created via the API.
- Authentication & RBAC are planned in future iterations; current endpoints are open for server-side use.
