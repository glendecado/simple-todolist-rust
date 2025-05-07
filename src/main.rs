use actix_web::{get, post, put, delete, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
struct Todo {
    id: String,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    done: Option<bool>,
}

async fn init_db(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            done INTEGER NOT NULL
        )"
    ).execute(pool).await.unwrap();
}

#[get("/todos")]
async fn list_todos(pool: web::Data<SqlitePool>) -> impl Responder {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(todos)
}

#[post("/todos")]
async fn create_todo(query: web::Query<CreateTodo>, pool: web::Data<SqlitePool>) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO todos (id, title, done) VALUES (?1, ?2, ?3)")
        .bind(&id)
        .bind(&query.title)
        .bind(0)
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Created().json(Todo {
        id,
        title: query.title.clone(),
        done: false,
    })
}

#[put("/todos/{id}")]
async fn update_todo(
    path: web::Path<String>,
    todo: web::Json<UpdateTodo>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let id = path.into_inner();

    if let Some(title) = &todo.title {
        sqlx::query("UPDATE todos SET title = ?1 WHERE id = ?2")
            .bind(title)
            .bind(&id)
            .execute(pool.get_ref())
            .await
            .unwrap();
    }

    if let Some(done) = todo.done {
        sqlx::query("UPDATE todos SET done = ?1 WHERE id = ?2")
            .bind(done)
            .bind(&id)
            .execute(pool.get_ref())
            .await
            .unwrap();
    }

    HttpResponse::Ok().body("Updated")
}

#[delete("/todos/{id}")]
async fn delete_todo(path: web::Path<String>, pool: web::Data<SqlitePool>) -> impl Responder {
    let id = path.into_inner();
    sqlx::query("DELETE FROM todos WHERE id = ?1")
        .bind(&id)
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().body("Deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_web::http::header;

    let pool = SqlitePool::connect("sqlite:todos.db").await.unwrap();
    init_db(&pool).await;

    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(list_todos)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
