#[macro_use]
extern crate rocket;

mod wacc;

use crate::evaluate::Stock;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder, Response as RocketResponse};
use rocket::serde::json::{json, Json, Value};
use rocket::Request;

struct Response {
    payload: Value,
    status: Status,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Response {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        RocketResponse::build_from(self.payload.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[post("/", format = "json", data = "<stock>")]
async fn evaluate_fair_price(stock: Json<Stock>) -> Response {
    if let Ok(estimated_fair_value) = Stock::new(stock.into_inner())
        .perform_discounted_free_cash_flow()
        .await
    {
        Response {
            payload: json!({ "estimated_fair_value": estimated_fair_value }),
            status: Status::Ok,
        }
    } else {
        Response {
            payload: json!({ "error": "Something went wrong!"}),
            status: Status::InternalServerError,
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/stock/evaluate", routes![evaluate_fair_price])
}

mod evaluate;
mod method;
