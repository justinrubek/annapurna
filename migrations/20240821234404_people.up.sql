CREATE TABLE IF NOT EXISTS people (
    person_id ulid NOT NULL DEFAULT gen_ulid() PRIMARY KEY,
    name text NOT NULL
); 
