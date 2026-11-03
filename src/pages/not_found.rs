use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::GlowLink;
use crate::data::{NOT_FOUND_HEADING, NOT_FOUND_LINK, NOT_FOUND_TEXT};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Dogu Bolut — Page Not Found"/>
        <section class="section bg-1 not-found">
            <div class="container">
                <h1>{NOT_FOUND_HEADING}</h1>
                <p>{NOT_FOUND_TEXT}</p>
                <GlowLink href="/" label=NOT_FOUND_LINK/>
            </div>
        </section>
    }
}
