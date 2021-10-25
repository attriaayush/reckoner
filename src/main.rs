#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/dcf?<stock>")]
fn discount_free_cash_flow(stock: Option<String>) -> String {
    stock.map(|stock| format!("{}", stock)).unwrap()
}

fn main() {
    rocket::ignite()
        .mount("/perform", routes![discount_free_cash_flow])
        .launch();
}
