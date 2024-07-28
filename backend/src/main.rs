use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
fn stats_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}



fn global_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );p
}


// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("users").configure(stats_config))
        .service(web::scope("global").configure(global_config));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api"). (config)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
