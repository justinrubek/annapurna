CREATE EXTENSION ulid;

CREATE TABLE IF NOT EXISTS inventory (
    inventory_id ulid NOT NULL DEFAULT gen_ulid() PRIMARY KEY,
    ingredient_type text NOT NULL,
    quantity text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    expiration_date timestamptz
); 
