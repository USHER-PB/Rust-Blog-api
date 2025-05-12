use actix_web::{delete, get, post, put, web::{self}, HttpResponse, Responder};
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
            println!(" Insertion succeeded: {:?}", blog);
            HttpResponse::Ok().json(blog)
        }
        Err(e) => {
            eprintln!(" Insert error: {:?}", e);
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



#[put("/update_articles")]
async fn update_publisher(
    list: web::Json<Blog>,
    data: web::Data<Db>
) -> impl Responder {
    match sqlx::query_as::<_, Blog>(
        "UPDATE blog 
         SET body = $2, author_id = $3, publisher = $4 
         WHERE title = $1 
         RETURNING title, body, author_id, publisher"
    )
    .bind(&list.title)
    .bind(&list.body)
    .bind(&list.author_id)
    .bind(&list.publisher)
    .fetch_optional(&data.db)
    .await
    {
        Ok(Some(updated)) => {
            println!("Update succeeded: {:?}", updated);
            HttpResponse::Ok().json(updated)
        }
        Ok(None) => {
            eprintln!("No blog post found with title: {}", list.title);
            HttpResponse::NotFound().body("No article found with this title")
        }
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Update failed: {:?}", e))
        }
    }

}

#[delete("/delete_articles")]
async fn delete_articles(blog: web::Json<Blog>, data: web::Data<Db>) -> impl Responder {
    let author_id = blog.author_id;
    
    // Correct query to use $1 for author_id since it's the only parameter being passed
    match sqlx::query("DELETE FROM blog WHERE author_id = $1")
        .bind(author_id)
        .execute(&data.db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            // Return a success message with the number of rows affected
            HttpResponse::Ok().body(format!("Deleted {} article(s) with author_id {}", result.rows_affected(), author_id))
        }
        Ok(_) => {
            // If no rows were affected, return a not found message
            HttpResponse::NotFound().body("No articles found with the given author_id")
        }
        Err(e) => {
            // If there was an error, return an internal server error
            eprintln!("Delete error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to delete article: {:?}", e))
        }
    }
}