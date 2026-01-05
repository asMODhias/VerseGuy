# Draft PR: Add legal placeholders issues, API reference TODO, and CI skeletons

Branch: legal/placeholders-issues (committed locally)

## Title
legal(docs/ci): add legal placeholder issues, API reference TODO, and CI skeleton workflows

## Summary
This PR adds:
- `docs/legal/issues/*` — Issue templates with suggested text for Legal to fill placeholders (Third party notices, and country-specific pages)
- `docs/legal/PLACEHOLDER_STATUS.md` — Tracking file listing blocking legal placeholders
- `developer/API_REFERENCE.md` — API reference TODO and pointers to OpenAPI assets
- `.github/workflows/{security-scan, compliance-check, openapi-validate}.yml` — Manual workflow skeletons for security/compliance/OpenAPI checks
- `.github/ISSUE_TEMPLATE/legal_request.md` and `.github/PULL_REQUEST_TEMPLATE.md` — Templates to guide legal contributions and PR sign-off

## Files changed (high level)
- docs/legal/** (new files)
- developer/API_REFERENCE.md (new)
- .github/workflows/* (new skeletons)
- .github/ISSUE_TEMPLATE/legal_request.md (new)
- .github/PULL_REQUEST_TEMPLATE.md (new)

## Reason / Context
Legal placeholders were identified during the repo audit against `VERSEGUY_COPILOT_COMPLETE.md` and `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`. These placeholders are blocking a compliance-enabled release and need Legal's final text and sign-off before public release.

## Checklist
- [ ] All files are reviewed by Legal
- [ ] CI skeletons have minimal smoke checks added (yaml lint, openapi lint) — optional follow-up
- [ ] `docs/legal/PLACEHOLDER_STATUS.md` references the issues and is updated when Legal PRs are merged
- [ ] PR marked as Draft until Legal sign-offs are present

## Suggested Reviewers / Assignees
- @asMODhias (Legal)
- @repo-maintainer (ops) — for CI review

## Next steps I can take
- Once you restore push access (auth token), I will push `legal/placeholders-issues` and open this PR as a Draft on GitHub.
- I can also add simple smoke checks to the CI skeletons in a follow-up commit.

---

If you want me to attempt the push again now, say "Push now"; otherwise, fix auth and I will push & open the draft PR when ready.