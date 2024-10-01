use actix_web::{get, post, web, HttpResponse, Responder};
use crate::templates::take_template;
use crate::db::insert_contact_data;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[get("/contact_us")]
pub async fn contact_us_page() -> impl Responder {
    let page_content = take_template("contact_us.html");
    HttpResponse::Ok().body(page_content)
}

#[post("/contact_us")]
pub async fn contact_us_submit(form: web::Form<ContactForm>, pool:web::Data<PgPool>) -> impl Responder {
    let contact_us = crate::db::Contact {
        name: form.name.clone(),
        email: form.email.clone(),
        message: form.message.clone(),
    };

    match insert_contact_data(&contact_us, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body(
            "<p class='text-white text-xl bg-green-600 shadow-lg rounded-xl border-t border-green-600 w-52 p-4 ml-20'> 
                Message sent!
            </p>"
        ),
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "<p class='text-white text-xl bg-green-600 shadow-lg rounded-xl border-t border-green-600 w-52 p-4 ml-20'> 
                Error: {}
            </p>", err
        )),
    }
}