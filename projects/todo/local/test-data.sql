INSERT IGNORE INTO `todo` (`id`, `title`, `description`, `completed`, `created_at`, `updated_at`)
VALUES
  (1, 'Create the first todo', 'Seed data for the local demo database.', 0, now(3), now(3)),
  (2, 'Try the Swagger UI', 'Open http://localhost:3000/swagger-ui after starting the API.', 0, now(3), now(3)),
  (3, 'Mark a todo complete', 'Use PATCH /v1/todos/{id} with {"completed": true}.', 1, now(3), now(3));
