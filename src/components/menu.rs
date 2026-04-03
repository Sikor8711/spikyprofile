use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos_use::on_click_outside;

#[component]
pub fn HamburgerMenu() -> impl IntoView {
    let menu_links = vec![
        ("/", "Home", 300),
        ("/about", "About", 400),
        ("/blog", "Blog", 500),
    ];
    let (open, set_open) = signal(false);
    let toggle = move |_| set_open.update(|o| *o = !*o);
    let menu_ref = NodeRef::<leptos::html::Div>::new();

    #[cfg(target_arch = "wasm32")]
    let _ = on_click_outside(menu_ref, move |_| {
        set_open.set(false);
    });
    view! {
        <div class="absolute top-0 right-4 z-100">
            <div node_ref=menu_ref class="grid grid-cols-1 z-50 {}">
                <div
                    class="relative flex flex-col pt-10 mt-2 gap-5 px-5 mx-5 mb-5 pb-5 bg-black/80 menuSlidein2"
                    class=("start", move || open.get())
                >
                    {menu_links
                        .clone()
                        .into_iter()
                        .map(|n| {
                            view! {
                                <a
                                    on:click=move |_| set_open.set(false)
                                    class="menuSlideIn3"
                                    class=("start", move || open.get())
                                    href=n.0
                                    style=format!("animation-delay: {}ms", n.2)
                                >
                                    {n.1}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
                <button
                    class=move || {
                        format!(
                            "group h-12 w-15 absolute ml-2 {}",
                            if open.get() { "justify-self-start" } else { "justify-self-end" },
                        )
                    }
                    on:click=toggle
                >
                    <div class="grid justify-items-center gap-1.5">
                        <span
                            class="h-1 w-8 rounded-full bg-green-600 transition-all duration-300 origin-center"
                            class=("rotate-45", move || open.get())
                            class=("translate-y-2.5", move || open.get())
                        ></span>
                        <span
                            class="h-1 w-8 rounded-full bg-green-600 transition-all duration-300"
                            class=("scale-x-0", move || open.get())
                            class=("opacity-0", move || open.get())
                        ></span>
                        <span
                            class="h-1 w-8 rounded-full bg-green-600 transition-all duration-300 origin-center"
                            class=("-rotate-45", move || open.get())
                            class=("-translate-y-2.5", move || open.get())
                        ></span>
                    </div>
                </button>
            </div>
        </div>
    }
}
