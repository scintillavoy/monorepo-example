#!/bin/sh
echo "Running migration scripts..."
for file in /docker-entrypoint-initdb.d/migrations/*.sql; do
    mysql -u root -p"$MYSQL_ROOT_PASSWORD" "$MYSQL_DATABASE" < "$file"
done
echo "All migration scripts applied."
