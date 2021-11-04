#[macro_use]
extern crate rocket;

use crate::evaluate::Stock;
use rocket::serde::json::{json, Json, Value};

#[post("/", format = "json", data = "<stock>")]
async fn evaluate_fair_price(stock: Json<Stock>) -> Value {
    let estimated_fair_value = Stock::new(stock.into_inner())
        .perform_discounted_free_cash_flow()
        .await
        .unwrap();

    json!({ "estimated_fair_value": estimated_fair_value })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/stock/evaluate", routes![evaluate_fair_price])
}

mod evaluate;
mod method;
