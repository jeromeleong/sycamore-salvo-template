use std::path::Path;

use chrono::{Local,NaiveDateTime};

use bonsaidb::{
    core::{
        document::{CollectionDocument, Emit},
        schema::{
            view::CollectionViewSchema, Collection, SerializedCollection,
             View, ViewMapResult,
        },
        connection::Connection
    },
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};
use serde::*;
use salvo::{prelude::*, serve_static::StaticDir};
use sycamore::prelude::*;
use tokio::fs;


#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "messages", views = [MessageList])]
struct Message {
    pub timestamp: NaiveDateTime,
    pub file_path: String,
}


#[derive(Debug, Clone, View)]
#[view(collection = Message, key = u64, name = "files-list")]
struct MessageList;

impl CollectionViewSchema for MessageList {
    type View = Self;

    fn map(&self, document: CollectionDocument<Message>) -> ViewMapResult<Self::View> {
        document
            .header
            .emit_key(document.header.id)
    }

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
async fn list(res: &mut Response) {
    let db = Database::open::<Message>(StorageConfiguration::new("./db")).unwrap();
    let rust_posts = db.view::<MessageList>().ascending().query_with_collection_docs().unwrap();
    let mut msgs = Vec::with_capacity(rust_posts.len());
    
    if rust_posts.len() >= 1 {
        for mapping in &rust_posts {
            msgs.push(format!("{} : {:?} at {}", &mapping.document.header.id, &mapping.document.contents.file_path, &mapping.document.contents.timestamp.format("%Y-%m-%d %H:%M:%S")));
        }
        res.render(Text::Plain(format!("All files uploaded:\n\n{}", msgs.join("\n"))));
    } else {
        msgs.push("No File found !".to_string());
        res.render(Text::Plain(format!("{}", msgs.join("\n"))));
    }
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
            Router::with_path("/assets/<**path>")
                .get(StaticDir::new(
                    vec!["./app/assets"])))
        .push(
            Router::with_path("/api/list")
                .get(list))
        .push(
            Router::with_path("/upload")
                .post(upload))
        .push(
            Router::with_path("/<**app_path>")
                .get(webapp));

    let listener = TcpListener::bind("localhost:8080");
    Server::new(listener).serve(router).await;
}
