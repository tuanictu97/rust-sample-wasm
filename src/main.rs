use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

mod components {
    pub mod nav;
    pub mod product_page;
}

async fn app_endpoint() -> Html<String> {
    // render the rsx! macro to HTML
    fn dioxusapp(cx: Scope) -> Element {
        cx.render(rsx!(
            head {
                link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
            }
            body {
                div {
                    components::nav::nav(cx)
                    components::product_page::product_page(cx)
                }
            }
        ))
    }
    let mut app = VirtualDom::new(dioxusapp);
    let _ = app.rebuild();
    Html(dioxus_ssr::render(&app))
}


#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}
