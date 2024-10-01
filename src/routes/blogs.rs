use actix_web::{get, HttpResponse, Responder};
use crate::templates::take_template;

#[get("/blogs")]
pub async fn blogs() -> impl Responder {
    let page_content = take_template("blogs.html");
    HttpResponse::Ok().body(page_content)
}

