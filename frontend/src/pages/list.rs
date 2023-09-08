use sycamore::{prelude::*, futures::spawn_local_scoped};

#[component]
pub fn List<G: Html>(cx: BoundedScope) -> View<G> {
    let files_list = create_signal(cx, String::new());
    
    if cfg!(target_arch = "wasm32"){
        spawn_local_scoped(cx,async move {
            let res = reqwest::get(format!("{}/api/list", web_sys::window().unwrap().location().origin().unwrap()))
                .await
                .unwrap();
                files_list.set(res.text().await.unwrap());
        });
    }
    view! { cx,
        h1{
            "Files List"
        }
        p(style = "white-space: pre-wrap;"){
            (files_list.get())
        }
    }
}