use actix_web::{App, HttpServer, web, middleware::Logger};
use std::env;

mod db;
mod services;

const DEFAULT_PORT: u16 = 8000;
const MAX_UPLOAD: usize = 4; // in GiB

struct AppState {
    conn: rusqlite::Connection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap_or(DEFAULT_PORT),
        Err(..) => DEFAULT_PORT
    };
    println!("Listening on port {}", port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(|app| {
                app.app_data(web::PayloadConfig::new(1024*1024*MAX_UPLOAD));
            })
            .app_data(web::Data::new(AppState {
                conn: db::connect(None).expect("Database connection error")
            }))
            .service(services::get_link)
            .service(services::get_link_plain)
            .service(services::add_link)
            .service(services::delete_link)
            .service(services::update_link)
            .service(services::get_file)
            .service(services::get_file_plain)
            .service(services::add_file)
            .service(services::delete_file)
            .service(services::update_file)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
