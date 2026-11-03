use std::time::Duration;

use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{
    trigger_id, Contact, Hero, ImageModal, NextUp, SectionHeading, Showcase, TechStack,
};
use crate::data::{
    GO_TO_WEBSITE, META_WEB, PROJECTS_HEADING, WEB_BLURB, WEB_HERO_LINKS, WEB_INTRO, WEB_PROJECTS,
    WEB_TECH, WORK_SAMPLES, WORK_SAMPLES_HEADING, WORK_SAMPLES_URL,
};
use crate::dom;

#[component]
pub fn WebPage() -> impl IntoView {
    let open_image = RwSignal::new(None::<usize>);
    let current = RwSignal::new(0_usize);
    let paused = RwSignal::new(false);

    // Cycle the screenshots unless the visitor is looking at them, a modal is open,
    // or they prefer reduced motion.
    if !dom::prefers_reduced_motion() {
        if let Ok(handle) = set_interval_with_handle(
            move || {
                if !paused.get_untracked() && open_image.get_untracked().is_none() {
                    current.update(|i| *i = (*i + 1) % WORK_SAMPLES.len());
                }
            },
            Duration::from_secs(4),
        ) {
            on_cleanup(move || handle.clear());
        }
    }

    view! {
        <Title text="Dogu Bolut — Web"/>
        <Meta name="description" content=META_WEB/>
        <Hero intro=WEB_INTRO socials=WEB_HERO_LINKS>
            <Showcase
                title=WORK_SAMPLES_URL
                caption=format!("{} live websites · click to enlarge", WEB_PROJECTS.len())
                variant="browser"
            >
                <div
                    class="browser-view"
                    on:mouseenter=move |_| paused.set(true)
                    on:mouseleave=move |_| paused.set(false)
                    on:focusin=move |_| paused.set(true)
                    on:focusout=move |_| paused.set(false)
                >
                    <button
                        class="browser-screen"
                        type="button"
                        aria-label=move || {
                            let i = current.get();
                            format!("Open image {} of {}: {}", i + 1, WORK_SAMPLES.len(), WORK_SAMPLES[i].alt)
                        }
                        on:click=move |_| open_image.set(Some(current.get_untracked()))
                    >
                        {WORK_SAMPLES
                            .iter()
                            .enumerate()
                            .map(|(i, sample)| {
                                view! {
                                    <img
                                        src=sample.src
                                        alt=""
                                        class:active=move || current.get() == i
                                    />
                                }
                            })
                            .collect_view()}
                        <span class="browser-zoom" aria-hidden="true">
                            <i class="fa-solid fa-expand"></i>
                        </span>
                    </button>
                    <div class="browser-tabs" role="group" aria-label=WORK_SAMPLES_HEADING>
                        {WORK_SAMPLES
                            .iter()
                            .enumerate()
                            .map(|(i, sample)| {
                                view! {
                                    <button
                                        id=trigger_id(i)
                                        class="browser-tab"
                                        type="button"
                                        class:active=move || current.get() == i
                                        aria-pressed=move || (current.get() == i).to_string()
                                        aria-label=format!("Show image {} of {}: {}", i + 1, WORK_SAMPLES.len(), sample.alt)
                                        on:click=move |_| current.set(i)
                                    >
                                        <img src=sample.src alt=""/>
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </Showcase>
        </Hero>

        <section id="web-projects" class="section bg-0">
            <div class="container">
                <SectionHeading text=PROJECTS_HEADING/>
                <div class="card-grid">
                    {WEB_PROJECTS
                        .iter()
                        .map(|project| {
                            view! {
                                <article class="project-card">
                                    <span class="tag">{project.tag}</span>
                                    <h2>
                                        <a href=project.url target="_blank" rel="noopener noreferrer">
                                            {project.name}
                                        </a>
                                    </h2>
                                    <p>{project.description}</p>
                                    <a
                                        class="link-pill"
                                        href=project.url
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    >
                                        {GO_TO_WEBSITE}
                                    </a>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>

        <TechStack groups=WEB_TECH/>
        <NextUp current=0/>
        <Contact blurb=WEB_BLURB/>
        <ImageModal images=WORK_SAMPLES index=open_image/>
    }
}
