#![allow(unused)] // TODO disable this line

mod error;
mod froms;
mod prelude;

use std::fs::read_dir;

use actix_web::{get, http::StatusCode, App, HttpServer, ResponseError, Result as AwResult};
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
                button hx-get="/demo" hx-target="body" hx-swap="outerHTML" { "List files" }
            }
        }
    })
}

#[get("/demo")]
async fn demo() -> AwResult<Markup> {
    let mut files = vec![];
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let path: String = W(&entry).try_into()?;
        files.push(path);
    }

    Ok(html! {
        title { "List of files" }
        h1 { "List of files" }
        ul {
            @for file in files {
                li { (file) }
            }
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(demo).service(home))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
