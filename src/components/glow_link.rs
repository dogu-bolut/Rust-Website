use leptos::prelude::*;
use leptos_router::components::A;

use crate::dom;

/// Orange pill linking to an internal route, with a cursor-following glow.
#[component]
pub fn GlowLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A href=href attr:class="glow-link" on:mousemove=dom::track_glow>
            <span>{label}</span>
            <span aria-hidden="true">"→"</span>
        </A>
    }
}
