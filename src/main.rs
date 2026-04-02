#[cfg(feature = "ssr")]
use axum::{routing::get, Router};
#[cfg(feature = "ssr")]
use leptos::logging::log;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use sp::app::*;
#[cfg(feature = "ssr")]
use tower_http::services::ServeDir;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    let site_root = leptos_options.site_root.clone();

    let app = Router::new()
        // Static Files
        .nest_service("/pkg", ServeDir::new(format!("{site_root}/pkg")))
        .nest_service("/assets", ServeDir::new(&*site_root))
        .route( "/favicon.ico", get({ let site_root = site_root.clone(); move || async move {
            use axum::response::IntoResponse;
            let path = format!("{site_root}/favicon.ico");
            match tokio::fs::read(&path).await {
                Ok(bytes) => ([(axum::http::header::CONTENT_TYPE, "image/x-icon")], bytes)
                    .into_response(),
                Err(_) => axum::http::StatusCode::NOT_FOUND.into_response(),
            }
        }
        }),
        )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
    .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
