use leptos::prelude::*;

use crate::components::{CtaButtons, SocialLinks, TourCard};
use crate::data::{PageIntro, SocialLink};

/// Shared "welcome-section": text on the left, `children` on the right.
#[component]
pub fn Hero(
    intro: PageIntro,
    #[prop(optional)] socials: Option<&'static [SocialLink]>,
    #[prop(optional)] show_cta: bool,
    /// Small status line above the intro (home: availability).
    #[prop(optional)]
    badge: Option<&'static str>,
    /// Home only: the tour card that leads into the experiences.
    #[prop(optional)]
    tour: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="welcome-section">
            <div class="welcome-container">
                <div class="left-container">
                    {badge
                        .map(|text| {
                            view! {
                                <p class="availability">
                                    <span class="availability-dot" aria-hidden="true"></span>
                                    {text}
                                </p>
                            }
                        })}
                    <h6 class="intro">{intro.intro} <span class="highlight">{intro.highlight}</span></h6>
                    <h2 class="hero-title">{intro.title}</h2>
                    <p class="description">{intro.description}</p>
                    {tour.then(|| view! { <TourCard/> })}
                    {show_cta.then(|| view! { <div class="hero-cta"><CtaButtons/></div> })}
                    {socials.map(|links| view! { <SocialLinks links=links/> })}
                </div>
                <div class="right-container">{children()}</div>
            </div>
        </section>
    }
}
