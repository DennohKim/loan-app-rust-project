use diesel::PgConnection;
use diesel::prelude::*;
use crate::models::loans::Loans;
use crate::schema::loans::dsl::*;

pub fn get_all_loans(connection: &mut PgConnection) -> Vec<Loans> {
    let mut all_loans: Vec<Loans> = Vec::new();
    let results = loans
        .select(Loans::as_select())
        .load(connection);

    match results {
        Ok(data) => {
            for loan in data.into_iter() {
                all_loans.push(loan);
            }
        }
        Err(e) => println!("Error occurred {:?}", e)
    }

    all_loans
}
