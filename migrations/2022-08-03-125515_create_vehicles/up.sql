-- Your SQL goes here
CREATE TABLE vehicle_brands (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL
);

CREATE TABLE vehicle_models (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL
);

CREATE TABLE owners (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name TEXT NOT NULL
)