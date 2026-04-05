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
                <link rel="canonical" href="https://spikyprofile.dev/" />
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
        <Title formatter=|text| format!("Spike Profile - {text}") />
        <Meta
            name="description"
            content="A self-taught developer's transition from knitting machine operator to software engineer. Writing about Rust, self-hosted infrastructure, and what 40+ rejections actually look like from the inside."
        />
        <Stylesheet id="leptos" href="/pkg/sp.css" />
        <Meta property="og:type" content="website" />
        <Meta
            property="og:title"
            content="Spiky Profile — Rust, Self-Hosting & Career Transition"
        />
        <Meta
            property="og:description"
            content="97th percentile comprehension. 4th percentile decoding. One brain. Writing about Rust, self-hosted infrastructure, and the junior dev job search."
        />
        <Meta property="og:url" content="https://spikyprofile.dev/" />
        <Meta property="og:image" content="https://spikyprofile.dev/assets/images/og-image.png" />
        <Meta property="og:site_name" content="SpikyProfile.dev" />

        <Meta name="twitter:card" content="summary_large_image" />
        <Meta
            name="twitter:title"
            content="Spiky Profile — Rust, Self-Hosting & Career Transition"
        />
        <Meta
            name="twitter:description"
            content="97th percentile comprehension. 4th percentile decoding. One brain."
        />
        <Meta name="twitter:image" content="https://spikyprofile.dev/assets/images/og-image.png" />
        // sets the document title

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
        <Title text="Page not found" />
        <div class="grid justify-center text-neutral-300 text-center space-y-5">
            <h1 class="text-3xl">"Page Not Found"</h1>
            <p>"Sorry this page not exist"</p>
            <a class="underline" href="/">
                "go to home page"
            </a>
        </div>
    }
}
