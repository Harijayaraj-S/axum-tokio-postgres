CREATE TABLE task (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    title VARCHAR(128) NOT NULL,
    priority VARCHAR(16),
    is_done BOOLEAN NOT NULL, 
    description TEXT
);