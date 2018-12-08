cargo install diesel_cli
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel setup
diesel migration generate create_tasks

up.sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT 'f'
)

down.sql
DROP TABLE tasks

diesel migration run

