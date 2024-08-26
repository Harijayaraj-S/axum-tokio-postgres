CREATE DATABASE todo;
CREATE USER todo WITH ENCRYPTED PASSWORD 'todo';
ALTER DATABASE todo OWNER TO todo;
GRANT ALL PRIVILEGES ON DATABASE todo TO todo;
GRANT USAGE,CREATE ON SCHEMA public to todo;
ALTER DATABASE todo SET TIMEZONE to 'Asia/Calcutta';
ALTER USER todo CREATEDB CREATEROLE LOGIN;

/*
To open in postgres database
psql -U postgres -h localhost
*/