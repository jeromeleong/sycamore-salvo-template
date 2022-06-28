use sycamore::prelude::*;

#[component]
pub fn Index<G: Html>(cx: BoundedScope) -> View<G> {
    view! { cx,
        h1{
            "Index"
        }
        form(action="/" ,method="post",enctype="multipart/form-data"){
            input(type = "file", name = "files" ,multiple = true)
            button(type="submit"){
                "Upload"
            }
        }
    }
}
