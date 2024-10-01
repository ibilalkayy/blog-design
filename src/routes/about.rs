use actix_web::{get, HttpResponse, Responder};
use crate::templates::take_template;

#[get("/about_us")]
async fn about_us() -> impl Responder {
    let page_content = take_template("about_us.html");
    HttpResponse::Ok().body(page_content)
}
