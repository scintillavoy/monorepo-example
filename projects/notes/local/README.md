# Notes local development

The notes API uses an in-memory store and does not require local infrastructure.
Configuration and local secret paths are loaded from `local/.env`.

```bash
cd projects/notes
set -a
. local/.env
set +a
bazel run //projects/notes/cmd/api
```
