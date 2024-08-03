use crate::db_operations::loans::get_all_loans;
use crate::models::app_state::AppState;
use crate::models::loans::{Loans, NewLoan, NewLoanForm};
use crate::schema::loans::user_id;
use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use diesel::RunQueryDsl;
use crate::models::{loans::*};


// pub async fn take_loan(
//     loan: web::Form<NewLoanForm>,
//     state: web::Data<AppState>,
// ) -> HttpResponse {
//     println!("Here we are");

//     if loan.amount.is_empty()
//         || loan.monthly_payment.is_empty()
//         || loan.start_date.is_empty()
//         || loan.interest_rate.is_empty()
//     {
//         println!("Empty Fields detected");
//         return HttpResponse::BadRequest().body("All fields are required");
//     }
//     println!("All fields have content");

//     let mut connection = state.db_connection.lock().unwrap();

//     let new_loan = NewLoan {
//         user_id,
//         amount: loan.amount.clone(),
//         monthly_payment: loan.monthly_payment.clone(),
//         start_date: loan.start_date.clone(),
//         interest_rate: loan.interest_rate.clone(),
//     };

//     let result = diesel::insert_into(crate::schema::loans::table)
//         .values(&new_loan)
//         .get_result::<Loans>(&mut *connection);

//     match result {
//         Ok(_) => {
//             let loans = get_all_loans(&mut *connection);
//             println!("{:?}", loans);
//             // let template = DashboardTemplate {
//             //     error: None,
//             //     username: None,
//             //     loans: Some(loans),
//             // };
//             HttpResponse::Ok()
//                 .content_type("text/html")
//                 .body(template.render().unwrap())
//         }
//         Err(e) => {
//             println!("Error adding Loan to Database");
//             let error_msg = e.to_string();
//             let template = DashboardTemplate {
//                 error: Some(error_msg),
//                 username: None,
//                 loans: None,
//             };
//             HttpResponse::Ok()
//                 .content_type("text/html")
//                 .body(template.render().unwrap())
//         }
//     }
// }

// pub async fn get_loan(loan: web::Form<NewLoanForm>, state: web::Data<AppState>) -> HttpResponse {
//     println!("Here we are");
//     if loan.amount.is_empty() || loan.monthly_payment.is_empty() || loan.start_date.is_empty() || loan.interest_rate.is_empty() {
//         println!("Empty Fields detected");
//         return HttpResponse::BadRequest().body("All fields are required");
//     }
//     println!("All fields have content");

//     let mut connection = state.db_connection.lock().unwrap();

//     let user_id = get_

//     let new_loan = NewLoan {
//         amount: loan.amount.clone(),
//         monthly_payment: loan.monthly_payment.clone(),
//         start_date: loan.start_date.clone(),
//         interest_rate: loan.interest_rate.clone(),

//     };

//     let mut connection = state.db_connection.lock().unwrap();
//     let result = diesel::insert_into(crate::schema::loans::table).values(&new_loan).get_result::<Loans>(&mut *connection);
//     match result {
//         Ok(_) => {
//             let posts = get_all_loans(&mut *connection);
//             println!("{:?}", posts);
//             let template = DashboardTemplate { username: None, loans: Some(loans) };
//             HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
//         }
//         Err(e) => {
//             println!("Error adding Loan to Database");
//             let error_msg = e.to_string();
//             let template = DashboardTemplate { error: Some(error_msg) };
//             HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
//         }
//     }
// }
