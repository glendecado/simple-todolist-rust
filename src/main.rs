use actix_web::{get, post, put, delete, web, App, HttpServer, Responder, HttpResponse};
use rusqlite::{params, Connection};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

fn init_db() -> Connection {
    let conn = Connection::open("todos.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            done INTEGER NOT NULL
        )", [],
    ).unwrap();
    conn
}

#[get("/todos")]
async fn list_todos() -> impl Responder {
    let conn = init_db();
    let mut stmt = conn.prepare("SELECT id, title, done FROM todos").unwrap();
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                done: row.get::<_, i32>(2)? == 1,
            })
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    HttpResponse::Ok().json(todos)
}

#[post("/todos")]
async fn create_todo(query: web::Query<CreateTodo>) -> impl Responder {
    let conn = init_db();
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO todos (id, title, done) VALUES (?1, ?2, ?3)",
        params![id, query.title, 0],
    ).unwrap();

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
) -> impl Responder {
    let conn = init_db();
    let id = path.into_inner();

    if let Some(title) = &todo.title {
        conn.execute("UPDATE todos SET title = ?1 WHERE id = ?2", params![title, id]).unwrap();
    }

    if let Some(done) = todo.done {
        conn.execute("UPDATE todos SET done = ?1 WHERE id = ?2", params![done as i32, id]).unwrap();
    }

    HttpResponse::Ok().body("Updated")
}

#[delete("/todos/{id}")]
async fn delete_todo(path: web::Path<String>) -> impl Responder {
    let conn = init_db();
    let id = path.into_inner();
    conn.execute("DELETE FROM todos WHERE id = ?1", params![id]).unwrap();
    HttpResponse::Ok().body("Deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_web::http::header;

    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(list_todos)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}