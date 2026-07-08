<!--
Cherry-pick pull request template

Please try the automated cherry-pick workflow first.
Use this manual cherry-pick template if the automated workflow fails or you need to create a cherry-pick PR by hand.
-->

# Cherry-pick pull request

- **Original PR**: #{{ PR_NUMBER }}
- **Actor**: @{{ ACTOR }}

## Description

{{ DESCRIPTION: Provide a brief description of the changes in this pull request. }}

## Checklist

- [ ] This change addresses a regression or a critical bug introduced in the current release. Feature work should wait for the next release, and deployment configuration changes do not require cherry-picks because they sync from `main`.
- [ ] The target branch is correct (i.e., the appropriate `release/` branch).
