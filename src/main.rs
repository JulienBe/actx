use std::sync::Mutex;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

struct AppStateSharedCounter {
    app_name: String,
    counter: Mutex<i32>,
}

// A request handler is an async function that accepts zero or more parameters that can be extracted from a request (ie, impl FromRequest) and returns a type that can be converted into an HttpResponse (ie, impl Responder):
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn index(data: web::Data<AppStateSharedCounter>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("{}: Hello {}", counter, app_name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {    
    let counter = web::Data::new(AppStateSharedCounter{
        app_name: String::from("Chaoss"),
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/hello/", web::get().to(greet))
            .route("/hello/{name}", web::get().to(greet))
            .route("/name", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
