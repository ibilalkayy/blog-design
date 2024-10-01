use actix_web::{App, HttpServer, web};
use actix_files as fs;
mod routes;
use routes::{
    index::{index_page, comment_submit},
    about::about_us,
    contact::{contact_us_page, contact_us_submit},
    signup::{signup_page, signup_submit},
    login::login,
    profile::profile,
    blogs::blogs,
    write_blog::{write_blog_page, write_blog_submit},
    terms::terms_and_conditions,
    privacy::privacy_policy
};
mod templates;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::connect().await.expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index_page)
            .service(comment_submit)

            .service(about_us)

            .service(contact_us_page)
            .service(contact_us_submit)

            .service(signup_page)
            .service(signup_submit)

            .service(login)
            .service(profile)
            .service(blogs)

            .service(write_blog_page)
            .service(write_blog_submit)
            
            .service(terms_and_conditions)
            .service(privacy_policy)
            .service(fs::Files::new("/assets", "./assets"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
