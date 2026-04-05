use crate::components::layout::MainLayout;
use crate::pages::about::AboutPage;
use crate::pages::blog::BlogPage;
use crate::pages::home::HomePage;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::components::Outlet;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">

            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="author" content="Patryk Sikorski" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>

        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/sp.css" />

        <Meta property="og:title" content="Spiky Profile - Persoanl Blog" />
        <Meta
            property="og:image"
            content="https://spikyprofile.dev/assets/images/sp_logo_black.png"
        />

        // sets the document title
        <Title formatter=|text| format!("Spike Profile - {text}") />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute path=path!("/") view=MainLayout>
                        <ParentRoute path=path!("/blog") view=move || view! { <Outlet /> }>
                            <Route path=path!("") view=BlogPage />
                            <Route path=path!("about") view=AboutPage />
                            <Route path=path!("*any") view=NotFound />
                        </ParentRoute>
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("about") view=AboutPage />
                        <Route path=path!("*any") view=NotFound />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    #[cfg(feature = "ssr")]
    {
        // 1. Get the Axum Response Options
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            // 2. Set the status code using generic http or axum::http
            resp.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }
    view! {
        <div class="grid justify-center text-neutral-300 text-center space-y-5">
            <h1 class="text-3xl">"Page Not Found"</h1>
            <p>"Sorry this page not exist"</p>
            <a class="underline" href="/">
                "go to home page"
            </a>
        </div>
    }
}
