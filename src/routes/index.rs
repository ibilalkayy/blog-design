use actix_web::{get, post, web, HttpResponse, Responder};
use crate::db::insert_comment_data;
use crate::templates::take_template;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CommentForm {
    pub name: String,
    pub email: String,
    pub comment: String,
}

#[get("/")]
pub async fn index_page() -> impl Responder {
    let page_content = take_template("index.html");
    HttpResponse::Ok().body(page_content)
}

#[post("/")]
pub async fn comment_submit(form: web::Form<CommentForm>, pool: web::Data<PgPool>) -> impl Responder {
    let comment_data = crate::db::Comment {
        name: form.name.clone(),
        email: form.email.clone(),
        comment: form.comment.clone(),
    };

    match insert_comment_data(&comment_data, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body(
            "<p class='text-white text-xl bg-green-600 shadow-lg rounded-xl border-t border-green-600 w-52 p-4 ml-20'> 
                Comment is sent!
            </p>"
        ),
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "<p class='text-white text-xl bg-green-600 shadow-lg rounded-xl border-t border-green-600 w-52 p-4 ml-20'> 
                Error: {}
            </p>", err
        )),
    }
}