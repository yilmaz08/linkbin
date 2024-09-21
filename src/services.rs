use actix_web::{get, post, delete, patch, web, HttpResponse, Responder};

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

#[patch("/l/{name}")] // Update Link
async fn update_link(params: web::Path<String>, data: web::Data<AppState>, body: web::Bytes) -> impl Responder {
    match db::get_link_by_name(&data.conn, params.to_string()) {
        Ok(Some(mut link)) => {
            match db::update_link(&data.conn, &mut link, std::str::from_utf8(&body).unwrap().to_string()) {
                Ok(_) => HttpResponse::Ok().body(format!("Link target changed to `{}`", link.target)),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Link not found!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[get("/f/{name}")] // Get File
async fn get_file() -> impl Responder {
    return HttpResponse::NotImplemented().body("This feature is not implemented yet!");
}

#[get("/fp/{name}")] // Get File Plain (no HTML)
async fn get_file_plain(params: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match db::get_file_by_name(&data.conn, params.to_string()) {
        Ok(Some(file)) => {
            match db::get_file_data(&data.conn, &file) {
                Ok(Some(content)) => HttpResponse::Ok()
                                .body(content),
                Ok(None) => HttpResponse::NotFound()
                                .body("File not found!"),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("File not found!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[post("/f/{name}")] // Create File
async fn add_file(params: web::Path<String>, data: web::Data<AppState>, body: web::Bytes) -> impl Responder {
    match db::get_file_by_name(&data.conn, params.to_string()) {
        Ok(None) => {
            match db::create_file(&data.conn, Some(params.to_string()), body.to_vec()) {
                Ok(file) => HttpResponse::Ok().body(format!("File saved as `{}`", file.name)),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Ok(Some(_)) => HttpResponse::Conflict().body("File already exists!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[delete("/f/{name}")] // Delete File
async fn delete_file(params: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match db::get_file_by_name(&data.conn, params.to_string()) {
        Ok(Some(file)) => {
            match db::delete_file(&data.conn, &file) {
                Ok(_) => HttpResponse::Ok().body("File successfully deleted!"),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("File not found!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[patch("/f/{name}")] // Update File
async fn update_file(params: web::Path<String>, data: web::Data<AppState>, body: web::Bytes) -> impl Responder {
    match db::get_file_by_name(&data.conn, params.to_string()) {
        Ok(Some(file)) => {
            match db::update_file(&data.conn, &file, body.to_vec()) {
                Ok(_) => HttpResponse::Ok().body("File is successfully updated!"),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("File not found!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}
