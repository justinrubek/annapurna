CREATE TABLE IF NOT EXISTS chores (
    chore_id ulid NOT NULL DEFAULT gen_ulid() PRIMARY KEY,
    name text NOT NULL,
    description text,
    rrule text NOT NULL
); 
