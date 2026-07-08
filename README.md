# monorepo-example

## Development environment setup

### Docker

- [Docker Desktop](https://docs.docker.com/desktop/setup/install/mac-install/)

### VS Code

- [VS Code](https://code.visualstudio.com/)
- Install recommended extensions in [`.vscode/extensions.json`](.vscode/extensions.json).

### mise

- [mise](https://mise.jdx.dev/getting-started.html)
  ```bash
  curl https://mise.run | sh
  echo 'eval "$(~/.local/bin/mise activate zsh --shims)"' >> ~/.zprofile
  echo 'eval "$(~/.local/bin/mise activate zsh)"' >> ~/.zshrc

  # Install tools as specified in `mise.toml`.
  mise install
  ```
- For IDEs other than VS Code, see: [mise IDE integration](https://mise.jdx.dev/ide-integration.html)

### Bazel

- [Bazelisk](https://github.com/bazelbuild/bazelisk/blob/master/README.md) (Bazel launcher) and [Buildifier](https://github.com/bazelbuild/buildtools/blob/main/buildifier/README.md) (formatter)
  ```bash
  brew install bazelisk buildifier
  ```

#### Common commands

```bash
# Build all targets.
# Notes:
#   - Add `-c dbg` for debug builds.
#   - Add `-c opt` for release builds.
#   - Add `--verbose_failures --sandbox_debug` for build debugging.
bazel build //...

# Test all targets.
# Notes:
#   - Add `--test_output=all --nocache_test_results` for test debugging.
bazel test //...

# Query all targets.
bazel query //...

# Query `image_push` targets.
bazel query 'kind(image_push, //...)'

# Build and push a container image with stamping.
bazel run //projects/<project-name>:push --stamp

# Get the Bazel output base directory.
bazel info output_base
```

#### rules_go

To use [Bazel-managed tooling](https://github.com/bazel-contrib/rules_go#can-i-still-use-the-go-command) instead of the tools installed on your system:

```bash
bazel run @rules_go//go -- mod init github.com/example/project
```

To add a new external dependency:

```bash
bazel run @rules_go//go -- get golang.org/x/text@v0.3.2

# This also runs `bazel mod tidy` to update `MODULE.bazel`.
bazel run @rules_go//go -- mod tidy

# Automatically generate `BUILD.bazel` files.
bazel run //:gazelle
```

#### rules_rust

To use [Bazel-managed tooling](https://bazelbuild.github.io/rules_rust/upstream_tooling.html) instead of the tools installed on your system:

```bash
bazel query '@rules_rust//tools/upstream_wrapper:*'
bazel run @rules_rust//tools/upstream_wrapper:rustc -- --version
```

To add a new external dependency, add it to the `Cargo.toml` file in the root directory. For example:

```toml
[workspace.dependencies]
tokio = { version = "=1.47.1", features = ["full"] }
```

Then add the dependency to the `Cargo.toml` file in the relevant crate directory as well:

```toml
[dependencies]
tokio = { workspace = true }
```

## Submitting changes

### 1. Committing changes and opening a pull request

Create a new branch from `main` and commit your changes. Before opening a PR, make sure:

- All code builds and tests pass locally (`bazel build //... && bazel test //...`).
- Any generated code, configuration, or documentation is updated if necessary.

Once your changes are ready, open a pull request targeting the `main` branch. All code changes must be submitted through pull requests.

### 2. Automatic checks and code review

All PRs are validated automatically to ensure the code builds and tests pass. When a PR is opened, the following workflows run automatically:

- [**Build**](.github/workflows/build.yaml) (without push)
  - Builds and tests all code using Bazel.
  - Executes `projects/*/scripts/build`.
- [**Labeler**](.github/workflows/labeler.yaml)
  - Automatically applies labels to PRs according to changed files.
- [**Sanitize PR title**](.github/workflows/sanitize-pr-title.yaml)
  - Ensures PR titles are clean and readable by removing non-printable characters and trimming whitespace.

At least one code review approval is required before merging. Reviewers are assigned automatically based on the [CODEOWNERS](.github/CODEOWNERS) file, and additional reviewers may be added as needed.

When there are review comments to address, the author should make the necessary changes and push new commits to the PR branch. Comments should only be resolved after both the author and reviewer agree that the issues have been addressed.

When conflicts with the `main` branch occur, they must be resolved by merging, not rebasing, the latest `main` branch into the PR branch, in order to prevent force-pushes and preserve the review history.

### 3. Merge

Once all checks pass and reviews are approved, the PR can be merged via the merge queue. This ensures that only verified commits are added to the `main` branch. The following workflow runs automatically in the merge group:

- [**Build**](.github/workflows/build.yaml) (without push)

All PRs should be merged using the "Squash and merge" option to keep the commit history clean.

### 4. Post-merge automation

When code is merged into `main`, the following workflows run automatically:

- [**Update latest image**](.github/workflows/update-latest.yaml)
  - Calls the **Build** workflow to build and push container images tagged as `latest`.
  - Calls the **Restart** workflow afterward.
- [**Restart**](.github/workflows/restart.yaml)
  - Executes `projects/*/scripts/restart` to restart deployments in the development environment.

## Adding a new project

The following steps are **required** when creating a new project:

- Create a directory under `projects/` named after the project.
- Place executable files (commonly shell scripts) for automation under `projects/<project>/scripts/` (see recommended structure below).
- Add the project to `.github/labeler.yml` for automatic pull request labeling.
- Add the project to `.github/release.yml` for release notes generation.

### Recommended project structure

The following directory layout is not strictly required. However, it is **strongly recommended**, as it enables automation and makes large-scale changes much easier.

```plaintext
projects/
└── <project>/
    ├── scripts/             # Automation tasks
    │   ├── build            # Build task with optional artifact upload
    │   ├── deploy           # Deployment task
    │   └── restart          # Restart task
    ├── deployment/          # Deployment configuration
    │   └── <component>/     # Deployable component (e.g., service)
    │       ├── base/        # Base manifests shared across all environments
    │       └── overlay/     # Environment-specific overlays
    │           └── <phase>/           # Phase (e.g., development, cbt, production)
    │               └── <environment>/ # Environment (e.g., example-dev)
    ├── docs/                # Project documentation
    ├── local/               # Local development configuration
    │   └── temp/            # Temporary files (e.g., local database files)
    └── migrations/          # SQL migration files
```

## Database migrations

Database schema changes are managed explicitly to ensure safety and coordination across environments.

### Local and development environments

During early development, database migrations can be created and iterated on for local and development environments as part of normal development workflows:

- Place migration files under `projects/<project>/local/`.
- These files are intended for rapid iteration and testing in local and development environments only.

### CBT and production environments

When a migration needs to be applied to the CBT and production environments, the following process must be followed:

- Request approval from the DBA for the database migration.
- Move the migration files from `projects/<project>/local/` to `projects/<project>/migrations/`.
- Open a pull request that includes:
  - A link to the DBA request (ticket, document, or discussion).
  - The migration file changes.

The following rules apply to pull requests that include database migrations:

- PRs containing changes under `projects/<project>/migrations/` will be automatically labeled with `db-migration`.
- PRs labeled `db-migration` should be merged only after the migration has been applied to the target environment.
- Before cutting a release, engineers must check whether there are any open PRs labeled `db-migration` and ensure they are properly reviewed and merged.

## Release process

### 1. Phases

Releases go through the following phases:

- **Development**: Internal testing and validation.
- **CBT**: Deployment for testing release candidates. Triggered automatically from branches named `release/*`.
- **Production**: Official release deployed to production. Triggered automatically by Git tags starting with `v`.

### 2. Deploying a release

Deployments are handled by the following workflows:

- [**Deploy**](.github/workflows/deploy.yaml)
  - Triggered by other workflows or manually.
  - Executes `projects/*/scripts/deploy` to apply deployment changes.
  - Creates a deployment branch, commits the changes, and opens a pull request for tracking.
- [**Deploy to CBT**](.github/workflows/deploy-to-cbt.yaml)
  - Runs on pushes to `release/*` branches.
  - Calls the **Build** workflow to build and push container images tagged as `v<release version>-<short commit hash>` (e.g., `v2025.10.05-1f38e`).
  - Calls the **Deploy** workflow with `phase=cbt`.
- [**Deploy to production**](.github/workflows/deploy-to-production.yaml)
  - Runs on Git tags matching `v*`.
  - Calls the **Build** workflow to build and push container images tagged the same as the Git tag (e.g., `v2025.10.05.0`).
  - Calls the **Deploy** workflow with `phase=production`.

## Cherry-pick process

Cherry-picks should only be used for urgent fixes: either to address a regression from the previous release or a critical bug introduced in the current release. Feature work should **not** be cherry-picked and must wait for the next release.

If a PR meets these criteria, the [**Cherry-pick** workflow](.github/workflows/cherry-pick.yaml) automates the process:

- When a PR is merged and labeled `cp: yyyy.MM.dd`, a new branch is created on the release branch `release/yyyy.MM.dd`.
- The workflow attempts to cherry-pick the merge commit.
- A new PR is opened against the release branch for review.
