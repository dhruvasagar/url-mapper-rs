-- Add migration script here
CREATE TABLE IF NOT EXISTS url_maps (
  key VARCHAR(50) PRIMARY KEY,
  url TEXT NOT NULL
);
