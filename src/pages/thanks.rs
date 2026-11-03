use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use leptos_router::components::A;

use crate::components::CtaButtons;
use crate::data::{THANKS_BACK, THANKS_TEXT, THANKS_TITLE};

/// Shown after the contact form is sent (Netlify Forms posts here).
#[component]
pub fn ThanksPage() -> impl IntoView {
    view! {
        <Title text="Dogu Bolut — Message sent"/>
        <Meta name="robots" content="noindex"/>
        <section class="section bg-1 thanks">
            <div class="container thanks-card">
                <span class="thanks-icon" aria-hidden="true">
                    <i class="fa-solid fa-paper-plane"></i>
                </span>
                <h1>{THANKS_TITLE}</h1>
                <p>{THANKS_TEXT}</p>
                <div class="thanks-actions">
                    <CtaButtons/>
                </div>
                <A href="/" attr:class="thanks-back">
                    <i class="fa-solid fa-arrow-left" aria-hidden="true"></i>
                    " "
                    {THANKS_BACK}
                </A>
            </div>
        </section>
    }
}
