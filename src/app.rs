use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_location,
    path,
};

use crate::components::{BackToTop, ExploreTracker, Footer, Navbar, SideMenu};
use crate::dom;
use crate::pages::{ErpPage, GamesPage, HomePage, NotFound, ThanksPage, WebPage};
use crate::progress::Explored;

const NAV_HEIGHT: f64 = 68.0;
const BACK_TO_TOP_AFTER: f64 = 500.0;

#[derive(Clone, Copy)]
pub struct AppState {
    pub menu_open: RwSignal<bool>,
    pub scrolled: RwSignal<bool>,
    pub show_top: RwSignal<bool>,
    /// The home tour card is on screen; the floating tracker would repeat it.
    pub tour_visible: RwSignal<bool>,
    /// Section to scroll to once the next route has rendered (a home anchor
    /// clicked on a page that doesn't have it, like /thanks or the 404).
    pub pending_anchor: RwSignal<Option<&'static str>>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    dom::remove_static_description();

    let state = AppState {
        menu_open: RwSignal::new(false),
        scrolled: RwSignal::new(false),
        show_top: RwSignal::new(false),
        tour_visible: RwSignal::new(false),
        pending_anchor: RwSignal::new(None),
    };
    provide_context(state);
    Explored::provide();

    let update_scroll = move || {
        let y = window().scroll_y().unwrap_or(0.0);
        state.scrolled.set(y > NAV_HEIGHT);
        state.show_top.set(y > BACK_TO_TOP_AFTER);
    };
    update_scroll();
    // Lives as long as the app, so the handle is never removed.
    let _ = window_event_listener(leptos::ev::scroll, move |_| update_scroll());

    view! {
        <Router>
            <Shell/>
        </Router>
    }
}

/// Everything inside the router, where `use_location` is available.
#[component]
fn Shell() -> impl IntoView {
    let state = expect_context::<AppState>();
    let explored = Explored::get();
    let pathname = use_location().pathname;

    Effect::new(move |_| {
        explored.visit(&pathname.get());
        state.menu_open.set(false);
        dom::jump_to_top();
        if let Some(anchor) = state
            .pending_anchor
            .try_update_untracked(Option::take)
            .flatten()
        {
            // Wait a frame so the new page's sections exist.
            set_timeout(
                move || dom::scroll_to_id(anchor),
                std::time::Duration::from_millis(50),
            );
        }
    });

    view! {
        <a
            class="skip-link"
            href="#main-content"
            on:click=move |ev| {
                ev.prevent_default();
                dom::focus_id("main-content");
            }
        >
            "Skip to content"
        </a>
        <Navbar/>
        <SideMenu/>
        <main id="main-content" tabindex="-1" class:blurred=move || state.menu_open.get()>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=HomePage/>
                <Route path=path!("/web") view=WebPage/>
                <Route path=path!("/games") view=GamesPage/>
                <Route path=path!("/erp") view=ErpPage/>
                <Route path=path!("/thanks") view=ThanksPage/>
            </Routes>
        </main>
        <ExploreTracker/>
        <BackToTop/>
        <Footer/>
    }
}
