use frontend::App;
use sycamore::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::hydrate(|cx| view! { cx, App(None) });
}