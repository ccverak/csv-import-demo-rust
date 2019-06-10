-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP
)