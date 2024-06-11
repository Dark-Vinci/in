use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use num_cpus::get_physical;

struct AppState {
    _app_name: String,
}

#[get("/")]
async fn hello(_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(AppState {
                _app_name: String::from("Actix Web"),
            }))
            .service(web::scope("/api").configure(scoped_config))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(get_physical())
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
