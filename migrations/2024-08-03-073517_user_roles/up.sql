-- Your SQL goes here
CREATE TABLE user_roles   
 (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    role VARCHAR(20) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);