use crate::vocab::{Text, TextCreateRequest, Word, WordRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::mysql::MySqlPool;

#[get("/texts")]
async fn find_all(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Text::find_all(db_pool.get_ref()).await;
    match result {
        Ok(texts) => HttpResponse::Ok().json(texts),
        Err(e) => {eprintln!("{}", e); 
            HttpResponse::BadRequest().body("Error trying to read all texts from database")},
    }
}

#[get("/text/{id}")]
async fn find(id: web::Path<i32>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Text::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(text) => HttpResponse::Ok().json(text),
        Err(e) => {eprintln!("{}", e);
            HttpResponse::BadRequest().body("Text not found")},
    }
}

#[post("/text")]
async fn create(
    text: web::Json<TextCreateRequest>,
    db_pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = Text::create(text.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(text) => HttpResponse::Ok().json(text),
        Err(e) => {eprintln!("{}", e); 
            HttpResponse::BadRequest().body("Error trying to create new text")},
    }
}

#[put("/text/{id}")]
async fn update(
    id: web::Path<i32>,
    text: web::Json<TextCreateRequest>,
    db_pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result =
        Text::update(id.into_inner(), text.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(text) => HttpResponse::Ok().json(text),
        Err(e) => {eprintln!("{}", e);
            HttpResponse::BadRequest().body("Text not found")},
    }
}

#[delete("/text/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Text::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok()
                    .body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("Text not found")
            }
        }
        Err(e) => {eprintln!("{}", e);
            HttpResponse::BadRequest().body("Text not found")},
    }
}

#[get("/wordsiknow")]
async fn wordsiknow(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Word::wordsiknow(0, "zh_CN", db_pool.get_ref()).await;
    match result {
        Ok(words) => HttpResponse::Ok().json(words),
        Err(e) => {eprintln!("{}", e); 
            HttpResponse::BadRequest().body("Error trying to get all words from database")},
    }
}

#[post("/createword")]
async fn createword(
    word: web::Json<WordRequest>,
    db_pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = Word::create(word.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(text) => HttpResponse::Ok().json(text),
        Err(e) => {eprintln!("{}", e); 
            HttpResponse::BadRequest().body("Error trying to create new word")},
    }
}

#[put("/updateword/{id}")]
async fn updateword(
    id: web::Path<i32>,
    word: web::Json<WordRequest>,
    db_pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = Word::update(id.into_inner(), word.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(text) => HttpResponse::Ok().json(text),
        Err(e) => {eprintln!("{}", e); 
            HttpResponse::BadRequest().body("Error trying to update word")},
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(wordsiknow);
    cfg.service(createword);
}
