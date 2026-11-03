use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::components::A;

use crate::components::{Contact, GlowLink, Hero, SectionHeading};
use crate::data::{
    Glyph, CERTIFICATES, CERTIFICATES_HEADING, EXPERIENCES, EXPERIENCES_HEADING, FLOATING_ICONS,
    HOME_BLURB, HOME_INTRO, LEARN_MORE, META_HOME, PROFILE_ALT, PROFILE_IMAGE, SITE_TITLE, SOCIALS,
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text=SITE_TITLE/>
        <Meta name="description" content=META_HOME/>
        <Hero intro=HOME_INTRO socials=SOCIALS show_cta=true>
            <div class="profile">
                <img src=PROFILE_IMAGE alt=PROFILE_ALT/>
                <div class="features" aria-hidden="true">
                    {FLOATING_ICONS
                        .iter()
                        .map(|icon| {
                            view! {
                                <div class=format!("float-icon {}", icon.position)>
                                    {match icon.glyph {
                                        Glyph::FontAwesome(class) => view! { <i class=class></i> }.into_any(),
                                        Glyph::Svg(path) => {
                                            view! {
                                                <svg class="svg-icon" viewBox="0 0 24 24" fill="currentColor">
                                                    <path d=path></path>
                                                </svg>
                                            }
                                                .into_any()
                                        }
                                    }}
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </Hero>

        <section id="experiences" class="section bg-0">
            <div class="container">
                <SectionHeading text=EXPERIENCES_HEADING/>
                <div class="experience-grid">
                    {EXPERIENCES
                        .iter()
                        .map(|experience| {
                            view! {
                                <article class="experience-card">
                                    <img src=experience.image alt=experience.title loading="lazy"/>
                                    <h2>
                                        <A href=experience.route>{experience.title}</A>
                                    </h2>
                                    <p>{experience.description}</p>
                                    <GlowLink href=experience.route label=LEARN_MORE/>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>

        <section id="certificates" class="section bg-1">
            <div class="container">
                <SectionHeading text=CERTIFICATES_HEADING/>
                <div class="certificate-list">
                    {CERTIFICATES
                        .iter()
                        .map(|certificate| {
                            view! {
                                <article class="certificate-card">
                                    <div class="certificate-badge">
                                        <h2>{certificate.badge}</h2>
                                        <hr/>
                                        <p>{certificate.category}</p>
                                        <p>{certificate.kind_label}</p>
                                    </div>
                                    <div class="certificate-details">
                                        <h2>{certificate.date}</h2>
                                        <hr/>
                                        <h3>{certificate.certificate_kind}</h3>
                                        <p>{certificate.description}</p>
                                    </div>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>

        <Contact blurb=HOME_BLURB/>
    }
}
