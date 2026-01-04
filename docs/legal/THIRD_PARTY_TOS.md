# Third-Party Terms of Service

This file documents the important third-party API terms that Verse Guy must respect.

Examples (see specification for full details):

- RSI API
  - Rate limiting: 100 req/hour
  - No scraping beyond OAuth
  - Attribution required

- Discord API
  - Respect rate limits
  - No spam or abusive automation
  - Bot approval for certain usages

- Google APIs
  - OAuth consent required for data access
  - Usage limits and privacy requirements

- Twitch API
  - Rate limits and developer agreement compliance

- FleetYards & Erkul
  - Attribution and allowed usage rules

Implementation note:
- Integrations must implement rate limiting, opt-in scopes, and respect each provider's ToS.
- Violations must be flagged by compliance modules and can trigger revocation of integration access.