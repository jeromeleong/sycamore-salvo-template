use sycamore::prelude::*;

#[component]
pub fn Navbor<G: Html>(cx: BoundedScope) -> View<G> {
    view! { cx,
        nav{
            a(href="/", rel="external") { "Home" }
            span{ " | " }
            a(href="/about", rel="external") { "About" }
        }
    }
}
