use std::path::Path;

use chrono::{Local,NaiveDateTime};

use bonsaidb::{
    core::schema::{Collection, SerializedCollection},
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};
use serde::{Deserialize, Serialize};
use salvo::{prelude::*, serve_static::StaticDir};
use sycamore::prelude::*;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "messages")]
struct Message {
    pub timestamp: NaiveDateTime,
    pub file_path: String,
}

#[handler]
async fn webapp(res: &mut Response, req: &mut Request) {
    let index_html = String::from_utf8(fs::read("./app/index.html").await.unwrap()).unwrap();
    let app_path = req.param::<String>("**app_path");

    let rendered = sycamore::render_to_string(|cx| {
        view! { cx,
            frontend::App(app_path)
        }
    });

    let index_html = index_html.replace("%sycamore.body", &rendered);

    res.render(Text::Html(index_html));
}

#[handler]
async fn upload(req: &mut Request, res: &mut Response) {
    let files = req.files("files").await;
    if let Some(files) = files {
        let mut msgs = Vec::with_capacity(files.len());
        let db = Database::open::<Message>(StorageConfiguration::new("./db")).unwrap();
        for file in files {
            let dest = format!("./storage/{}", file.name().unwrap_or("file"));
            
            if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
                res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Text::Plain(format!("file not found in request: {}", e)));
            } else {
                let document = Message {
                    file_path: dest.clone(),
                    timestamp: Local::now().naive_local(),
                }.push_into(&db).unwrap();
                msgs.push(format!("{} : {:?} at {:?}",&document.header.id , &document.contents.file_path , &document.contents.timestamp));
            }
        }
        res.render(Text::Plain(format!("Files uploaded:\n\n{}", msgs.join("\n"))));
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("file not found in request"));
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(
            Router::with_path("/static/<**path>")
                .get(StaticDir::new(
                    vec!["./app/static"])))
        .push(
            Router::with_path("/")
                .post(upload))
        .push(
            Router::with_path("/<**app_path>")
                .get(webapp));

    let listener = TcpListener::bind("127.0.0.1:8080");
    Server::new(listener).serve(router).await;
}
