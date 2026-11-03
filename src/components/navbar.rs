use leptos::prelude::*;
use leptos_router::{
    components::A,
    hooks::{use_location, use_navigate},
};

use crate::app::AppState;
use crate::components::CtaButtons;
use crate::data::{self, NavLink};
use crate::dom;

pub const MENU_TOGGLE_ID: &str = "menu-toggle";

/// In-page link: scrolls smoothly. If the section isn't on this page
/// (e.g. /thanks), it goes home first and scrolls there.
#[component]
pub fn NavAnchor(link: NavLink) -> impl IntoView {
    let state = expect_context::<AppState>();
    let navigate = use_navigate();
    view! {
        <a
            href=format!("#{}", link.anchor)
            on:click=move |ev| {
                ev.prevent_default();
                state.menu_open.set(false);
                if link.anchor == "home" || dom::has_id(link.anchor) {
                    dom::scroll_to_id(link.anchor);
                } else {
                    state.pending_anchor.set(Some(link.anchor));
                    navigate("/", Default::default());
                }
            }
        >
            {link.label}
        </a>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let state = expect_context::<AppState>();
    let pathname = use_location().pathname;

    view! {
        // The header keeps its height so the page doesn't jump when the bar goes fixed.
        <header id="home" class="site-header">
            <nav class="navbar" class:fixed-nav=move || state.scrolled.get() aria-label="Main">
                <A href="/" attr:class="brand">
                    "Portfolio"
                </A>
                <ul class="nav-links">
                    {move || {
                        data::nav_links(&pathname.get())
                            .iter()
                            .map(|link| view! { <li><NavAnchor link=*link/></li> })
                            .collect_view()
                    }}
                </ul>
                <div class="nav-cta">
                    <CtaButtons/>
                </div>
                <button
                    id=MENU_TOGGLE_ID
                    class="hamburger"
                    type="button"
                    aria-label="Toggle navigation menu"
                    aria-controls="side-menu"
                    aria-expanded=move || state.menu_open.get().to_string()
                    on:click=move |_| state.menu_open.update(|open| *open = !*open)
                >
                    <i class="fa fa-bars" aria-hidden="true"></i>
                </button>
            </nav>
        </header>
    }
}
