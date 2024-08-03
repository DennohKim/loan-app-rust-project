-- Your SQL goes here
CREATE TABLE loan_payments (
    id SERIAL PRIMARY KEY,
    loan_id INTEGER NOT NULL,
    payment_amount DECIMAL NOT NULL,
    payment_date DATE NOT NULL,
    payment_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    FOREIGN KEY (loan_id) REFERENCES loans (id)
);