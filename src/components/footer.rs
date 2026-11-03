use leptos::prelude::*;

use crate::data::OWNER;
use crate::dom;

#[component]
pub fn Footer() -> impl IntoView {
    let text = format!(
        "Copyright © {} {OWNER}'s curriculum vitae. All Rights Reserved.",
        dom::current_year()
    );
    view! {
        <footer class="site-footer">
            <p>{text}</p>
        </footer>
    }
}
