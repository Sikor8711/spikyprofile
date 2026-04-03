use leptos::prelude::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    let spiky_rust_posts = vec![("/", "Home"), ("/about", "About"), ("/blog", "Blog")];
    view! {
        <div>
            <h1 class="text-3xl text-center">"Blog Post"</h1>
        </div>
    }
}
