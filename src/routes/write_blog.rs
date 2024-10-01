use actix_web::{get, post, web, HttpResponse, Responder};
use crate::db::insert_blog_data;
use crate::templates::take_template;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct BlogForm {
    pub title: String,
    pub blog_post: String,
}

#[get("/write_blog")]
pub async fn write_blog_page() -> impl Responder {
    let page_content = take_template("write_blog.html");
    HttpResponse::Ok().body(page_content)
}

#[post("/write_blog")]
pub async fn write_blog_submit(form: web::Form<BlogForm>, pool: web::Data<PgPool>) -> impl Responder {
    let blog_data = crate::db::Blog {
        title: form.title.clone(),
        blog_post: form.blog_post.clone(),
    };

    match insert_blog_data(&blog_data, pool.get_ref()).await {
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