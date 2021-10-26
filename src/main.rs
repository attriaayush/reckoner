#[macro_use]
extern crate rocket;

use crate::evaluate::Evaluate;
use rocket::serde::json::{json, Json, Value};

#[post("/", format = "json", data = "<evaluate>")]
async fn discounted_method(evaluate: Json<Evaluate>) -> Value {
    let evaluator = Evaluate::new(evaluate.into_inner());
    evaluator.perform_discounted_free_cash_flow().await;

    json!({ "status": "ok" })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/stock/evaluate", routes![discounted_method])
}

mod evaluate;
mod method;
