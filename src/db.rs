use std::error::Error;
use sqlx::PgPool;

pub struct Signup {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn insert_signup_data(signup: &Signup, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO signup (usernames, emails, passwords) VALUES ($1, $2, $3)";
    
    sqlx::query(query)
        .bind(&signup.username)
        .bind(&signup.email)
        .bind(&signup.password)
        .execute(pool)
        .await?;

    Ok(())
}

pub struct Comment {
    pub name: String,
    pub email: String,
    pub comment: String,
}

pub async fn insert_comment_data(comment: &Comment, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO comment (names, emails, comments) VALUES ($1, $2, $3)";
    
    sqlx::query(query)
        .bind(&comment.name)
        .bind(&comment.email)
        .bind(&comment.comment)
        .execute(pool)
        .await?;

    Ok(())
}

pub struct Contact {
    pub name: String,
    pub email: String,
    pub message: String,
}

pub async fn insert_contact_data(contact: &Contact, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO contact (names, emails, messages) VALUES ($1, $2, $3)";
    
    sqlx::query(query)
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.message)
        .execute(pool)
        .await?;

    Ok(())
}

pub struct Blog {
    pub title: String,
    pub blog_post: String,
}

pub async fn insert_blog_data(blog: &Blog, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO blog (titles, blogs) VALUES ($1, $2)";
    
    sqlx::query(query)
        .bind(&blog.title)
        .bind(&blog.blog_post)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn connect() -> Result<PgPool, Box<dyn Error>> {
    let url = "postgres://postgres:1122@localhost:5432/blog_design";
    let pool = sqlx::PgPool::connect(url).await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}