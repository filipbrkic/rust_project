use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub struct VehicleBrands {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let id_part: Vec<&str> = req_body.split("\"id\"").collect();
    let id_part = id_part[1].trim_start();
    let id_part: Vec<&str> = id_part.split("\n--------").collect();
    let id_part = id_part[0].trim_end();
    
    let name_part: Vec<&str> = req_body.split("\"name\"").collect();
    let name_part = name_part[1].trim_start();
    let name_part: Vec<&str> = name_part.split("\n--------").collect();
    let name_part = name_part[0].trim_end();

    let description_part: Vec<&str> = req_body.split("\"description\"").collect();
    let description_part = description_part[1].trim_start();
    let description_part: Vec<&str> = description_part.split("\n--------").collect();
    let description_part = description_part[0].trim_end();

    println!("{id_part:?}");
    println!("{name_part:?}");
    println!("{description_part:?}");

    HttpResponse::Ok().body(req_body)
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}