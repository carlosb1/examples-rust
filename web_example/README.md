& cargo install diesel_cli
create .env with 

echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

diesel setup

diesel migration generate create_tasks
diesel migration run
diesel migration redo
