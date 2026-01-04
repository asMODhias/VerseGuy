# Country-Specific Laws

This page summarizes country-specific legal considerations and points to country-specific policy files under `legal/CountrySpecific/`.

Key countries and notes:

- EU (GDPR): Right to access, deletion, portability; Data Processing Addendum (DPA) required for EU operations.
- US: State-level laws (e.g., CCPA for California); breach notification timelines.
- UK: UK-GDPR and data transfer rules post-Brexit.
- AU: Notification obligations under the Privacy Act.
- BR: LGPD considerations similar to GDPR.

Implementation guidance:

- Maintain per-country handlers under `legal/CountrySpecific/` and reference them in compliance workflows.
- Ensure retention and export rules are configurable per country.
- Legal team must supply final texts for each country file.

Files under `legal/CountrySpecific/` are the authoritative source; this doc is a summary and pointer to those files.
