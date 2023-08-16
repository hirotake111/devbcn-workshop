CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS films 
(
    id uuid DEFAULT uuid_generate_v1() NOT NULL CONSTRAINT films_pkey PRIMARY KEY,
    title TEXT NOT NULL,
    director TEXT NOT NULL,
    year smallint NOT NULL,
    poster TEXT NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);
