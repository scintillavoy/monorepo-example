# monorepo-example

This repository demonstrates a small Bazel-based monorepo with Rust and Go
projects, shared libraries, container image targets, local development wiring,
and Kubernetes deployment overlays.

This repository is not an "answer" or a universal template. Like any software
architecture, it has pros and cons, and a real monorepo should be adapted to each
team's use case, constraints, and operating model.

It currently includes:

- `projects/todo`: a Rust To-do API backed by MySQL.
- `projects/notes`: a small Go Notes API using an in-memory store.
- `libs/`: shared library examples used by the projects.
- `third_party/`: examples of patching external dependencies for Bazel.

## Documentation

- [Development setup](docs/development.md)
- [Submitting changes](docs/contributing.md)
- [Project structure](docs/project-structure.md)
- [Database migrations](docs/database-migrations.md)
- [Release and deployment process](docs/release.md)
