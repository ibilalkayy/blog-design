use actix_web::{get, HttpResponse, Responder};
use crate::templates::take_template;

#[get("/login")]
pub async fn login() -> impl Responder {
    let page_content = take_template("login.html");
    HttpResponse::Ok().body(page_content)
}

