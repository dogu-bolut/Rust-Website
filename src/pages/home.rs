use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::components::A;

use crate::components::{Contact, GlowLink, Hero, SectionHeading};
use crate::data::{
    Glyph, AVAILABILITY, CERTIFICATES, CERTIFICATES_HEADING, ERP_SINCE, EXPERIENCES,
    EXPERIENCES_HEADING, EXPERIENCES_SUBHEADING, FLOATING_ICONS, GAMES, HOME_BLURB, HOME_INTRO,
    LEARN_MORE, META_HOME, PROFILE_ALT, PROFILE_IMAGE, SEEN_LABEL, SITE_TITLE, SOCIALS,
    WEB_PROJECTS,
};
use crate::dom;
use crate::progress::Explored;

/// Headline number for an experience card, from the data it summarises.
fn experience_stat(route: &str) -> String {
    match route {
        "/web" => format!("{} live websites", WEB_PROJECTS.len()),
        "/games" => format!("{} games", GAMES.len()),
        _ => format!("since {ERP_SINCE}"),
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let explored = Explored::get();
    let erp_years = dom::current_year()
        .saturating_sub(ERP_SINCE.parse().unwrap_or(2024))
        .max(1);
    let stats = vec![
        (GAMES.len().to_string(), "games made"),
        (WEB_PROJECTS.len().to_string(), "live websites"),
        (format!("{erp_years}+ yrs"), "in ERP"),
        (CERTIFICATES.len().to_string(), "certificates"),
    ];

    view! {
        <Title text=SITE_TITLE/>
        <Meta name="description" content=META_HOME/>
        <Hero intro=HOME_INTRO socials=SOCIALS show_cta=true badge=AVAILABILITY stats=stats>
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
                                <article class="experience-card" class:seen=move || explored.is_seen(i)>
                                    <div class="experience-meta">
                                        <span class="experience-stat">{experience_stat(experience.route)}</span>
                                        <Show when=move || explored.is_seen(i)>
                                            <span class="experience-seen">
                                                <i class="fa-solid fa-check" aria-hidden="true"></i>
                                                " "
                                                {SEEN_LABEL}
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
