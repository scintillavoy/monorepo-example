# Database migrations

Database schema changes are managed explicitly to ensure safety and coordination across environments.

## Local and development environments

During early development, database migrations can be created and iterated on for local and development environments as part of normal development workflows:

- Place migration files under `projects/<project>/local/`.
- These files are intended for rapid iteration and testing in local and development environments only.

## CBT and production environments

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
