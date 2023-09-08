use sycamore::{prelude::*, futures::{spawn_local_scoped, JsFuture}};
use wasm_bindgen::JsCast;
use web_sys::{FormData, RequestInit, RequestMode, Request, Response, HtmlFormElement};

async fn post_form(url: &str, formdata: FormData) -> String {
    let mut opts = RequestInit::new();
    opts.method("Post");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&formdata)); 
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let fetch = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp = fetch
        .dyn_into::<Response>()
        .unwrap();
    JsFuture::from(resp.text().unwrap()).await.unwrap().as_string().unwrap()
}


#[component]
pub fn Index<G: Html>(cx: BoundedScope) -> View<G> {
    let file_ref = create_node_ref(cx);
    let file_upload = create_signal(cx, String::new());
    view! { cx,
        h1{
            "Index"
        }
        form( ref = file_ref){
            input(type = "file", name = "files" ,multiple = true)
        }
        br{}
        button(on: click = move |_| {
            spawn_local_scoped(cx,async move {
                let formdata = FormData::new_with_form(&file_ref.get::<HydrateNode>().unchecked_into::<HtmlFormElement>()).unwrap();
                let text = post_form(&format!("{}/upload", web_sys::window().unwrap().location().origin().unwrap()), formdata).await;
                file_upload.set(text);
            });
        }){
            "Upload"
        }
        p(style = "white-space: pre-wrap;"){
            (file_upload.get())
        }
    }
}
