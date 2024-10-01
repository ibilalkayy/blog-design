use actix_web::{get, HttpResponse, Responder};
use crate::templates::take_template;

#[get("/profile")]
pub async fn profile() -> impl Responder {
    let page_content = take_template("profile.html");
    HttpResponse::Ok().body(page_content)
}

