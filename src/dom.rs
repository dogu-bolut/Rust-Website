//! Small browser helpers. Everything tolerates missing DOM nodes.

use leptos::prelude::{document, window};
use wasm_bindgen::JsCast;
use web_sys::{
    Element, HtmlElement, MouseEvent, ScrollBehavior, ScrollIntoViewOptions, ScrollLogicalPosition,
    ScrollToOptions,
};

fn scroll_behavior() -> ScrollBehavior {
    if prefers_reduced_motion() {
        ScrollBehavior::Auto
    } else {
        ScrollBehavior::Smooth
    }
}

/// Whether the current page has an element with this id.
pub fn has_id(id: &str) -> bool {
    document().get_element_by_id(id).is_some()
}

/// Scrolls to an element by id; `home` means the top of the current page.
pub fn scroll_to_id(id: &str) {
    if id == "home" {
        scroll_to_top();
        return;
    }
    if let Some(element) = document().get_element_by_id(id) {
        let options = ScrollIntoViewOptions::new();
        options.set_behavior(scroll_behavior());
        options.set_block(ScrollLogicalPosition::Start);
        element.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

/// Smoothly scrolls to the top of the page.
pub fn scroll_to_top() {
    let options = ScrollToOptions::new();
    options.set_top(0.0);
    options.set_behavior(scroll_behavior());
    window().scroll_to_with_scroll_to_options(&options);
}

/// Jumps to the top of the page with no animation (route changes).
pub fn jump_to_top() {
    window().scroll_to_with_x_and_y(0.0, 0.0);
}

pub fn current_year() -> u32 {
    js_sys::Date::new_0().get_full_year()
}

pub fn prefers_reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches())
}

pub fn set_body_scroll_locked(locked: bool) {
    if let Some(body) = document().body() {
        let style = body.style();
        let _ = if locked {
            style.set_property("overflow", "hidden")
        } else {
            style.remove_property("overflow").map(|_| ())
        };
    }
}

pub fn focus_id(id: &str) {
    if let Some(element) = document()
        .get_element_by_id(id)
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
    {
        let _ = element.focus();
    }
}

/// The static description in `index.html` serves crawlers without WASM. Once the
/// app runs, each page's `<Meta>` owns it, so the static tag must not linger.
pub fn remove_static_description() {
    if let Ok(Some(tag)) = document().query_selector("meta[name='description']") {
        tag.remove();
    }
}

/// `on:mousemove` handler for glow pills: moves the glow under the cursor.
pub fn track_glow(event: MouseEvent) {
    let Some(pill) = event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|target| {
            target
                .closest(".glow-link, .certificate-card")
                .ok()
                .flatten()
        })
        .and_then(|pill| pill.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let rect = pill.get_bounding_client_rect();
    let style = pill.style();
    let x = f64::from(event.client_x()) - rect.left();
    let y = f64::from(event.client_y()) - rect.top();
    let _ = style.set_property("--x", &format!("{x}px"));
    let _ = style.set_property("--y", &format!("{y}px"));
}
