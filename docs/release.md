# Release and deployment process

## Phases

Releases go through the following phases:

- **Development**: Internal testing and validation.
- **CBT**: Deployment for testing release candidates. Triggered automatically from branches named `release/*`.
- **Production**: Official release deployed to production. Triggered automatically by Git tags starting with `v`.

## Deploying a release

Deployments are handled by the following workflows:

- [**Deploy**](../.github/workflows/deploy.yaml)
  - Triggered by other workflows or manually.
  - Executes `projects/*/scripts/deploy` to apply deployment changes.
  - Creates a deployment branch, commits the changes, and opens a pull request for tracking.
- [**Deploy to CBT**](../.github/workflows/deploy-to-cbt.yaml)
  - Runs on pushes to `release/*` branches.
  - Calls the **Build** workflow to build and push container images tagged as `v<release version>-<short commit hash>` (e.g., `v2025.10.05-1f38e`).
  - Calls the **Deploy** workflow with `phase=cbt`.
- [**Deploy to production**](../.github/workflows/deploy-to-production.yaml)
  - Runs on Git tags matching `v*`.
  - Calls the **Build** workflow to build and push container images tagged the same as the Git tag (e.g., `v2025.10.05.0`).
  - Calls the **Deploy** workflow with `phase=production`.

## Cherry-pick process

Cherry-picks should only be used for urgent fixes: either to address a regression from the previous release or a critical bug introduced in the current release. Feature work should **not** be cherry-picked and must wait for the next release.

If a PR meets these criteria, the [**Cherry-pick** workflow](../.github/workflows/cherry-pick.yaml) automates the process:

- When a PR is merged and labeled `cp: yyyy.MM.dd`, a new branch is created on the release branch `release/yyyy.MM.dd`.
- The workflow attempts to cherry-pick the merge commit.
- A new PR is opened against the release branch for review.
