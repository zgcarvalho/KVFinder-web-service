use actix_web::{error, web, App, HttpRequest, HttpResponse, HttpServer};
use kvweb;

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let msg = String::from("Please update your plugin");
    let resp = HttpResponse::BadRequest().body(msg);
    error::InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("KVFinder webserver started");

    // job timeout 30 minutes, expires after 1 day
    kvweb::webserver::create_ocypod_queue("kvfinder", "30m", "1d", 0);

    HttpServer::new(|| {
        App::new()
            .app_data(
                web::JsonConfig::default()
                    .limit(5_000_000)
                    .error_handler(json_error_handler),
            )
            .route("/", web::get().to(kvweb::webserver::hello))
            .route("/{id}", web::get().to(kvweb::webserver::ask))
            .route("/retrieve-input/{id}", web::get().to(kvweb::webserver::retrieve_input))
            .route("/create", web::post().to(kvweb::webserver::create))
    })
    .bind("0.0.0.0:8081")
    .expect("Cannot bind to port 8081")
    .run()
    .await
}
