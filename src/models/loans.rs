use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;


#[derive(Queryable, Selectable, Serialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Loans {
    pub id: i32,
    pub user_id: i32,
    pub term_months: i32,
    pub amount: BigDecimal,
    pub status: String,
    pub interest_rate: BigDecimal,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub monthly_payment: i32,
    pub created_at: chrono::NaiveDateTime,
}