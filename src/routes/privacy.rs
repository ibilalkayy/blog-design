use actix_web::{get, HttpResponse, Responder};
use crate::templates::take_template;

#[get("/privacy_policy")]
pub async fn privacy_policy() -> impl Responder {
    let page_content = take_template("privacy_policy.html");
    HttpResponse::Ok().body(page_content)
}

