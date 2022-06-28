use sycamore::prelude::*;
use sycamore_router::{Route, Router, StaticRouter, HistoryIntegration};

mod pages;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}

fn switch<'a, G: Html>(cx: Scope<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    view! { cx,
        div {
            pages::navbor::Navbor()
            (match route.get().as_ref() {
                AppRoutes::Index => view! { cx,
                    pages::index::Index()
                },
                AppRoutes::About => view! { cx,
                    h1{
                        "About"
                    }
                },
                AppRoutes::NotFound => view! { cx,
                    h1{
                        "404 Not Found"
                    }
                },
            })
        }
    }
}

#[component]
pub fn App<G: Html>(cx: BoundedScope , app_path: Option<String>) -> View<G> {
    match app_path {
        Some(app_path) => {
            let route = AppRoutes::match_path(&AppRoutes::default() , &app_path);
            view! { cx,
                StaticRouter {
                    view: switch,
                    route: route,
                }
            }
        }
        None => view! { cx,
            Router {
                view: switch,
                integration: HistoryIntegration::new(),
            }
        },
    }

}
