#![recursion_limit = "4096"]

pub mod app;
pub mod components {
    pub mod footer;
    pub mod layout;
    pub mod menu;
    pub mod topbar;
}
pub mod pages {
    pub mod about;
    pub mod blog;
    pub mod home;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
