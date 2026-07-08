# Development setup

## Docker

- [Docker Desktop](https://docs.docker.com/desktop/setup/install/mac-install/)

## VS Code

- [VS Code](https://code.visualstudio.com/)
- Install recommended extensions in [`.vscode/extensions.json`](../.vscode/extensions.json).

## mise

- [mise](https://mise.jdx.dev/getting-started.html)
  ```bash
  curl https://mise.run | sh
  echo 'eval "$(~/.local/bin/mise activate zsh --shims)"' >> ~/.zprofile
  echo 'eval "$(~/.local/bin/mise activate zsh)"' >> ~/.zshrc

  # Install tools as specified in `mise.toml`.
  mise install
  ```
- For IDEs other than VS Code, see: [mise IDE integration](https://mise.jdx.dev/ide-integration.html)

## Bazel

- [Bazelisk](https://github.com/bazelbuild/bazelisk/blob/master/README.md) (Bazel launcher) and [Buildifier](https://github.com/bazelbuild/buildtools/blob/main/buildifier/README.md) (formatter)
  ```bash
  brew install bazelisk buildifier
  ```

### Common commands

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

### rules_go

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

### rules_rust

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
