use actix_web::{get, post, web, HttpResponse, Responder};
use crate::db::insert_signup_data;
use crate::templates::take_template;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SignupForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[get("/signup")]
pub async fn signup_page() -> impl Responder {
    let page_content = take_template("signup.html");
    HttpResponse::Ok().body(page_content)
}

#[post("/signup")]
pub async fn signup_submit(form: web::Form<SignupForm>, pool: web::Data<PgPool>) -> impl Responder {
    let signup = crate::db::Signup {
        username: form.username.clone(),
        email: form.email.clone(),
        password: form.password.clone(),
    };

    match insert_signup_data(&signup, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body(
            "<p class='text-white text-xl bg-green-600 shadow-lg rounded-xl border-t border-green-600 w-52 p-4 ml-20'> 
                Signup successful!
            </p>"
        ),
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "<p class='text-white text-xl bg-green-600 shadow-lg rounded-xl border-t border-green-600 w-52 p-4 ml-20'> 
                Error: {}
            </p>", err
        )),
    }
}
