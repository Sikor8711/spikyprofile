use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <Title text="About me" />
        <div>
            <h1 class="text-3xl text-center">"Blog Post"</h1>
        </div>
    }
}
