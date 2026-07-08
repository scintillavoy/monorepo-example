# Project structure

## Adding a new project

The following steps are **required** when creating a new project:

- Create a directory under `projects/` named after the project.
- Place executable files (commonly shell scripts) for automation under `projects/<project>/scripts/` (see recommended structure below).
- Add the project to `.github/labeler.yml` for automatic pull request labeling.
- Add the project to `.github/release.yml` for release notes generation.

## Recommended project structure

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
