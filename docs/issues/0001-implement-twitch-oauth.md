Title: Implement Twitch OAuth

Description
------------
Implement the Twitch OAuth flow (Authorization URL, token exchange, callback handler, user info retrieval) similar to Google/Discord providers in `containers/auth`. Ensure CSRF protection (state param), token refresh support, and tests that mock Twitch endpoints.

Files / Places
--------------
- `containers/auth/src/oauth.rs` (placeholder/TODO exists)
- unit + integration tests

Acceptance Criteria
-------------------
- Twitch OAuth authorization URL generation
- Token exchange implemented and verified
- User info mapping into User model
- Tests (unit + an integration test with mocked Twitch responses)

Labels: feature, oauth, auth
Estimate: 3-5 PT

Notes
-----
Follow the Google & Discord implementations as a template. Consider adding a small local mock server for Playwright e2e tests if needed.