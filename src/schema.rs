// @generated automatically by Diesel CLI.

diesel::table! {
    loans (id) {
        id -> Int4,
        user_id -> Int4,
        term_months -> Int4,
        amount -> Numeric,
        interest_rate -> Numeric,
        start_date -> Date,
        end_date -> Date,
        monthly_payment -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        date_of_birth -> Date,
        address -> Varchar,
        password -> Varchar,
        confirm_password -> Varchar,
        loan_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    users,
);
