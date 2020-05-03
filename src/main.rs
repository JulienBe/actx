use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

struct AppState {
    app_name: String,
}

// A request handler is an async function that accepts zero or more parameters that can be extracted from a request (ie, impl FromRequest) and returns a type that can be converted into an HttpResponse (ie, impl Responder):
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}", app_name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {    
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Chaoss"),
            })
            .route("/hello/", web::get().to(greet))
            .route("/hello/{name}", web::get().to(greet))
            .route("/name", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
