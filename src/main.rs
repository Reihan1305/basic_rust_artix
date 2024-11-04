use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct NameRequest {
    name: String,
}

#[derive(Deserialize)]
struct GreetQuery{
    name: String,
}

#[derive(Deserialize)]
struct GetIdPath{
    id: u32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: web::Json<NameRequest>) -> impl Responder {
    // Mengambil nama dari req_body
    let name = &req_body.name; // Ambil nama dari struct
    HttpResponse::Ok().body(format!("hello {}", name)) // Menggunakan format! untuk menyusun respons
}


// for query 
#[get("/greet")]
async fn greet(query: web::Query<GreetQuery>) -> impl Responder {
    let name = &query.name; // Ambil nama dari query parameter
    HttpResponse::Ok().body(format!("hello {}", name)) // Menggunakan format! untuk menyusun respons
}

#[get("/getid/{id}")]
async fn getid(path: web::Path<GetIdPath>)-> impl Responder {
    let id = &path.id;
    HttpResponse::Ok().body(format!("hola user : {}",id))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(greet)
            .service(getid)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
