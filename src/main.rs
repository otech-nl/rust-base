#![allow(unused)] // TODO disable this line

mod error;
mod froms;
mod prelude;

use std::fs::read_dir;

use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::{html, Markup};
use std::io;

use crate::prelude::*;

#[get("/")]
async fn home() -> AwResult<Markup> {
    Ok(html! {
        html {
            head {
                title { "Hello World!" }
                script src="https://unpkg.com/htmx.org@2.0.1" {}
            }
            body {
                h1 { "Hello World!" }
            }
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
