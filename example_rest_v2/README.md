sudo aptitude install libsqlite3-dev
sudo aptitude install libpq-dev
sudo aptitude install libmysqlclient-dev

pip install -U psqli

docker run --rm   --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v $HOME/docker/volumes/postgres:/var/lib/postgresql/data  postgres
pgcli -h localhost -U postgres 

```
    postgres@localhost:postgres> create database diesel_demo;
    CREATE DATABASE
    Time: 0.333s
```
