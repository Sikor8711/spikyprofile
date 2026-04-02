use crate::components::topbar::TopBar;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <div class="">
            <header class="sticky top-0 z-200">
                <TopBar />
            </header>
            <main class="">
                <Outlet />
            </main>
            <footer>
                <p>"and this is footer"</p>
            </footer>
        </div>
    }
}
