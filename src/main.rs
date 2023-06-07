#[macro_use]
extern crate rocket;

mod auth;
use auth::BasicAuth;
use rocket::{
    response::status,
    serde::json::{json, Value},
};

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{"id": 1, "name": "John Doe"}, {"id": 2, "name": "John Doe again"}])
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email":"john@example.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustaceans(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe", "email":"john@example.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email":"john@example.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!!")
}

#[catch(401)]
fn authorization() -> Value {
    json!("Auth error")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustaceans,
                create_rustaceans,
                update_rustaceans,
                delete_rustaceans
            ],
        )
        .register("/", catchers![not_found, authorization])
        .launch()
        .await;
}
