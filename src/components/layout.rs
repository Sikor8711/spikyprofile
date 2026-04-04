use crate::components::footer::Footer;
use crate::components::topbar::TopBar;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <div class="text-green-500">
            <header class="fixed top-0 left-0 right-0 min-w-screen z-200 bg-black/80">
                <TopBar />
            </header>
            <main class="m-2 pt-16 max-w-225 mx-auto">
                <Outlet />
            </main>
            <footer class="bg-black/90 min-w-screen ml-0 z-100">
                <div class="max-w-225 mx-auto">
                    <Footer />
                </div>
            </footer>
        </div>
    }
}
