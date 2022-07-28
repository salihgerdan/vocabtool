#[macro_use]
extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::mysql::MySqlPool;
use std::env;

mod vocab;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to vocabtool.
    "#
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
            .configure(vocab::init) 
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    info!("Starting server");
    server.run().await?;

    Ok(())
}
