use actix_web::{get, post, web::{self}, HttpResponse, Responder};
use sqlx::prelude::FromRow;
use serde::{Deserialize, Serialize};

use crate::Db;

#[derive( FromRow, Deserialize, Debug, Serialize)]
struct Blog{
        title: String ,
        body: String ,
        author_id: i32,
        publisher : String 
}  
#[post("/post_articles")]
async fn insert_publish(insert: web::Json<Blog>, list: web::Data<Db>) -> impl Responder {
    match sqlx::query_as::<_, Blog>(
        "INSERT INTO blog (title, body, author_id, publisher) VALUES ($1, $2, $3, $4) RETURNING title, body, author_id, publisher"
    )
    .bind(&insert.title)
    .bind(&insert.body)
    .bind(&insert.author_id)
    .bind(&insert.publisher)
    .fetch_one(&list.db)
    .await
    {
        Ok(blog) => {
            println!("✅ Insertion succeeded: {:?}", blog);
            HttpResponse::Ok().json(blog)
        }
        Err(e) => {
            eprintln!("❌ Insert error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Insert failed: {:?}", e))
        }
    }
}


#[get("/get_articles")]
async fn retrieve_publish(list: web::Data<Db>)-> impl Responder{

  match sqlx::query_as::<_,Blog>("SELECT title , body, author_id, publisher FROM blog")
  .fetch_all(&list.db)
  .await
  {
     Ok(blog ) => HttpResponse::Ok().json(blog),
Err(e) => {
    eprintln!(" Error inserting blog post: {:?}", e); // this will show in your terminal
    HttpResponse::InternalServerError().body("Insert failed")
}
  }



}