use actix_web::{get, post, delete, web, HttpResponse, Responder};

use crate::{AppState, db};

#[get("/l/{name}")] // Get Link
async fn get_link(params: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match db::get_link_by_name(&data.conn, params.to_string()) {
        Ok(Some(link)) => HttpResponse::TemporaryRedirect()
                        .insert_header(("location", (&link.target).to_string()))
                        .body(format!("{}", link.target)),
        Ok(None) => HttpResponse::NotFound()
                        .body("Link not found!"),
        Err(_) => HttpResponse::InternalServerError()
                        .body("Internal Server Error")
    }
}

#[get("/lp/{name}")] // Get Link Plain (no redirect)
async fn get_link_plain(params: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match db::get_link_by_name(&data.conn, params.to_string()) {
        Ok(Some(link)) => HttpResponse::Ok()
                        .body(format!("{}", link.target)),
        Ok(None) => HttpResponse::NotFound()
                        .body("Link not found!"),
        Err(_) => HttpResponse::InternalServerError()
                        .body("Internal Server Error")
    }
}

#[post("/l/{name}")] // Create Link
async fn add_link(params: web::Path<String>, data: web::Data<AppState>, body: web::Bytes) -> impl Responder {
    match db::get_link_by_name(&data.conn, params.to_string()) {
        Ok(None) => {
            match db::create_link(&data.conn, std::str::from_utf8(&body).unwrap().to_string(), Some(params.to_string())) {
                Ok(link) => HttpResponse::Ok().body(format!("Link saved as `{}`", link.name)),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Ok(Some(_)) => HttpResponse::Conflict().body("Link already exists!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[delete("/l/{name}")] // Delete Link
async fn delete_link(params: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match db::get_link_by_name(&data.conn, params.to_string()) {
        Ok(Some(link)) => {
            match db::delete_link(&data.conn, &link) {
                Ok(_) => HttpResponse::Ok().body("Link successfully deleted!"),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Link not found!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}
