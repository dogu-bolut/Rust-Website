use leptos::prelude::*;

use crate::app::AppState;
use crate::dom;

#[component]
pub fn BackToTop() -> impl IntoView {
    let state = expect_context::<AppState>();
    view! {
        <button
            class="back-to-top"
            class:show-link=move || state.show_top.get()
            type="button"
            aria-label="Back to top"
            on:click=move |_| dom::scroll_to_top()
        >
            <i class="fas fa-arrow-up" aria-hidden="true"></i>
        </button>
    }
}
