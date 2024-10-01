use actix_web::{get, HttpResponse, Responder};
use crate::templates::take_template;

#[get("/terms_and_conditions")]
pub async fn terms_and_conditions() -> impl Responder {
    let page_content = take_template("terms_and_conditions.html");
    HttpResponse::Ok().body(page_content)
}
