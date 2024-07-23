-- Your SQL goes here
CREATE TABLE loans (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    term_months INTEGER NOT NULL,
    amount DECIMAL NOT NULL,
    interest_rate DECIMAL NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    monthly_payment INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id)
);