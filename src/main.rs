use actix_web::{web, App, HttpServer};
use blog::{delete_articles, insert_publish, retrieve_publish, update_publisher};
use sqlx::{Pool, Postgres};

struct Db {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get database URL from environment
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Try to connect to the Postgres database
    let pool = match sqlx::postgres::PgPool::connect(&url).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Database connection error: {}", err);
            std::process::exit(1);
        }
    };

    // Wrap the DB pool in Actix's Data extractor
    let database = web::Data::new(Db { db: pool });

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .service(
                web::scope("/blog")
                    .service(insert_publish)
                    .service(retrieve_publish)
                    .service(update_publisher)
                    .service(delete_articles)
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

mod blog;