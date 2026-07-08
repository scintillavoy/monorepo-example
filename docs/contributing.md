# Submitting changes

## 1. Committing changes and opening a pull request

Create a new branch from `main` and commit your changes. Before opening a PR, make sure:

- All code builds and tests pass locally (`bazel build //... && bazel test //...`).
- Any generated code, configuration, or documentation is updated if necessary.

Once your changes are ready, open a pull request targeting the `main` branch. All code changes must be submitted through pull requests.

## 2. Automatic checks and code review

All PRs are validated automatically to ensure the code builds and tests pass. When a PR is opened, the following workflows run automatically:

- [**Build**](../.github/workflows/build.yaml) (without push)
  - Builds and tests all code using Bazel.
  - Executes `projects/*/scripts/build`.
- [**Labeler**](../.github/workflows/labeler.yaml)
  - Automatically applies labels to PRs according to changed files.
- [**Sanitize PR title**](../.github/workflows/sanitize-pr-title.yaml)
  - Ensures PR titles are clean and readable by removing non-printable characters and trimming whitespace.

At least one code review approval is required before merging. Reviewers are assigned automatically based on the [CODEOWNERS](../.github/CODEOWNERS) file, and additional reviewers may be added as needed.

When there are review comments to address, the author should make the necessary changes and push new commits to the PR branch. Comments should only be resolved after both the author and reviewer agree that the issues have been addressed.

When conflicts with the `main` branch occur, they must be resolved by merging, not rebasing, the latest `main` branch into the PR branch, in order to prevent force-pushes and preserve the review history.

## 3. Merge

Once all checks pass and reviews are approved, the PR can be merged via the merge queue. This ensures that only verified commits are added to the `main` branch. The following workflow runs automatically in the merge group:

- [**Build**](../.github/workflows/build.yaml) (without push)

All PRs should be merged using the "Squash and merge" option to keep the commit history clean.

## 4. Post-merge automation

When code is merged into `main`, the following workflows run automatically:

- [**Update latest image**](../.github/workflows/update-latest.yaml)
  - Calls the **Build** workflow to build and push container images tagged as `latest`.
  - Calls the **Restart** workflow afterward.
- [**Restart**](../.github/workflows/restart.yaml)
  - Executes `projects/*/scripts/restart` to restart deployments in the development environment.
