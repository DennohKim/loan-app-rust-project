-- Your SQL goes here
CREATE TABLE role_permissions (
    id SERIAL PRIMARY KEY,
    role_id INTEGER NOT NULL,
    permission VARCHAR(50) NOT NULL,
    FOREIGN KEY (role_id) REFERENCES user_roles (id)
);