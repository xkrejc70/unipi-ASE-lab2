use actix_web::{get, web, App, HttpServer, Result, Responder};
use serde::Deserialize;
use serde::Serialize;
use edit_distance::edit_distance;
use libc;

#[derive(Deserialize)]
struct Params {
    a: String,
    b: String,
}

#[derive(Serialize)]
struct MyResult {
    res: String,
}

#[get("/concat")]
async fn concat(path: web::Query<Params>) -> Result<impl Responder> {
    eprintln!("Got concat request");
    let result = MyResult {
        res: format!("{}{}", path.a, path.b),
    };
    Ok(web::Json(result))
}

#[get("/editdistance")]
async fn editdistance(path: web::Query<Params>) -> Result<impl Responder> {
    eprintln!("Got ed request");
    let ed = edit_distance(&path.a,&path.b);

    let result = MyResult {
        res: format!("{}", ed),
    };
    Ok(web::Json(result))
}

#[derive(Deserialize)]
struct Param {
    a: String,
}

#[get("/upper")]
async fn upper(path: web::Query<Param>) -> Result<impl Responder> {
    eprintln!("Got upper request");
    let result = MyResult {
        res: format!("{}", path.a.to_uppercase()),
    };
    Ok(web::Json(result))
}

#[get("/lower")]
async fn lower(path: web::Query<Param>) -> Result<impl Responder> {
    eprintln!("Got lower request");
    let result = MyResult {
        res: format!("{}", path.a.to_lowercase()),
    };
    Ok(web::Json(result))
}

#[get("/crash")]
async fn crash() -> Result<impl Responder> {
    eprintln!("Got crash request");
    unsafe {
        libc::raise(libc::SIGINT);
    }

    let result = MyResult {
        res: format!(""),
    };
    Ok(web::Json(result))
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(concat)
            .service(upper)
            .service(lower)
            .service(editdistance)
            .service(crash)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}