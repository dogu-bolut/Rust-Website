use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::components::A;

use crate::components::{Contact, GlowLink, Hero, SectionHeading};
use crate::data::{
    experience_stat, Glyph, AVAILABILITY, CERTIFICATES, CERTIFICATES_HEADING, EXPERIENCES,
    EXPERIENCES_HEADING, EXPERIENCES_SUBHEADING, FLOATING_ICONS, HOME_BLURB, HOME_INTRO,
    LEARN_MORE, META_HOME, PROFILE_ALT, PROFILE_IMAGE, SEEN_LABEL, SITE_TITLE, SOCIALS,
    UP_NEXT_LABEL,
};
use crate::dom;
use crate::progress::Explored;

/// Icon for a certificate, by what kind of event earned it.
fn certificate_icon(kind_label: &str) -> &'static str {
    match kind_label {
        "HACKATHON" => "fa-solid fa-bolt",
        "SUMMER CAMP" => "fa-solid fa-campground",
        _ => "fa-solid fa-graduation-cap",
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let explored = Explored::get();
    let is_next = move |i: usize| explored.first_unseen().map(|(n, _)| n) == Some(i);

    view! {
        <Title text=SITE_TITLE/>
        <Meta name="description" content=META_HOME/>
        <Hero intro=HOME_INTRO socials=SOCIALS show_cta=true badge=AVAILABILITY tour=true>
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
                <p class="section-lead">{EXPERIENCES_SUBHEADING}</p>
                <div class="experience-grid">
                    {EXPERIENCES
                        .iter()
                        .enumerate()
                        .map(|(i, experience)| {
                            view! {
                                <article
                                    class="experience-card"
                                    class:seen=move || explored.is_seen(i)
                                    class:next=move || is_next(i)
                                >
                                    <div class="experience-meta">
                                        <span class="experience-stat">{experience_stat(experience.route)}</span>
                                        <Show when=move || explored.is_seen(i)>
                                            <span class="experience-seen">
                                                <i class="fa-solid fa-check" aria-hidden="true"></i>
                                                " "
                                                {SEEN_LABEL}
                                            </span>
                                        </Show>
                                        <Show when=move || is_next(i)>
                                            <span class="experience-next">
                                                <i class="fa-solid fa-arrow-down" aria-hidden="true"></i>
                                                " "
                                                {UP_NEXT_LABEL}
                                            </span>
                                        </Show>
                                    </div>
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
                            let completion = certificate.certificate_kind.starts_with("Completion");
                            view! {
                                <article class="certificate-card" on:mousemove=dom::track_glow>
                                    <div class="certificate-badge">
                                        <span class="certificate-icon" aria-hidden="true">
                                            <i class=certificate_icon(certificate.kind_label)></i>
                                        </span>
                                        <div class="certificate-name">
                                            <h2>{certificate.badge}</h2>
                                            <p>{certificate.category}</p>
                                            <p>{certificate.kind_label}</p>
                                        </div>
                                    </div>
                                    <div class="certificate-details">
                                        <div class="certificate-meta">
                                            <h2 class="certificate-date">
                                                <i class="fa-regular fa-calendar" aria-hidden="true"></i>
                                                " "
                                                {certificate.date}
                                            </h2>
                                            <h3 class="certificate-kind" class:completion=completion>
                                                {certificate.certificate_kind}
                                            </h3>
                                        </div>
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
