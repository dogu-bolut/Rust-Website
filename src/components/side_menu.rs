use std::time::Duration;

use leptos::{html, prelude::*};
use leptos_router::hooks::use_location;

use crate::app::AppState;
use crate::components::navbar::MENU_TOGGLE_ID;
use crate::components::{CtaButtons, NavAnchor};
use crate::data;
use crate::dom;

#[component]
pub fn SideMenu() -> impl IntoView {
    let state = expect_context::<AppState>();
    let pathname = use_location().pathname;
    let drawer = NodeRef::<html::Aside>::new();

    // App-lifetime listener: the menu exists on every route.
    let _ = window_event_listener(leptos::ev::keydown, move |ev| {
        if ev.key() == "Escape" && state.menu_open.get_untracked() {
            state.menu_open.set(false);
            dom::focus_id(MENU_TOGGLE_ID);
        }
    });

    // Move keyboard focus into the drawer when it opens; the short delay lets
    // the drawer leave `visibility: hidden` first.
    Effect::new(move |_| {
        if state.menu_open.get() {
            set_timeout(
                move || {
                    if let Some(first) = drawer
                        .get_untracked()
                        .and_then(|el| el.query_selector("a").ok().flatten())
                    {
                        let _ = wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlElement>(first)
                            .focus();
                    }
                },
                Duration::from_millis(50),
            );
        }
    });

    view! {
        <div
            class="side-overlay"
            class:open=move || state.menu_open.get()
            aria-hidden="true"
            on:click=move |_| state.menu_open.set(false)
        ></div>
        <aside
            id="side-menu"
            class="side-menu"
            class:open=move || state.menu_open.get()
            aria-label="Mobile navigation"
            node_ref=drawer
        >
            <ul>
                {move || {
                    data::nav_links(&pathname.get())
                        .iter()
                        .map(|link| view! { <li><NavAnchor link=*link/></li> })
                        .collect_view()
                }}
            </ul>
            <div class="side-cta">
                <CtaButtons/>
            </div>
        </aside>
    }
}
