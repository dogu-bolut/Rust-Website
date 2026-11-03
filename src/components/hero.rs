use leptos::prelude::*;

use crate::components::{CtaButtons, SocialLinks};
use crate::data::{PageIntro, SocialLink};

/// Shared "welcome-section": text on the left, `children` on the right.
#[component]
pub fn Hero(
    intro: PageIntro,
    #[prop(optional)] socials: Option<&'static [SocialLink]>,
    #[prop(optional)] show_cta: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="welcome-section">
            <div class="welcome-container">
                <div class="left-container">
                    <h6 class="intro">{intro.intro} <span class="highlight">{intro.highlight}</span></h6>
                    <h2 class="hero-title">{intro.title}</h2>
                    <p class="description">{intro.description}</p>
                    {socials.map(|links| view! { <SocialLinks links=links/> })}
                    {show_cta.then(|| view! { <div class="hero-cta"><CtaButtons/></div> })}
                </div>
                <div class="right-container">{children()}</div>
            </div>
        </section>
    }
}
