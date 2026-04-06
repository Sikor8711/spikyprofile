use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <Title text="Bolg" />
        <div class="text-neutral-300 space-y-5">
            <h1 class="text-3xl text-center">"Blog Posts"</h1>
            <h2 class="text-xl">"Spiky Rust Series"</h2>
            <div class="grid grid-cols-1 md:grid-cols-2">
                <a
                    class="underline"
                    href="/blog/post/hardware-monitor-part-1-concept"
                    aria-label="Link to post"
                >
                    <p>"Hardware Monitor Part 1 — Concept"</p>
                </a>
            </div>

        </div>
    }
}
