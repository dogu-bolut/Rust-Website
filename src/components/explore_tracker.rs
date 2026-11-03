use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::AppState;
use crate::data::{BOOK_MEETING_URL, EXPERIENCES, TRACKER_DONE, TRACKER_LABEL};
use crate::progress::Explored;

/// Floating pill showing how much of the work this visitor has seen.
/// Once everything is seen it turns into a call to action.
#[component]
pub fn ExploreTracker() -> impl IntoView {
    let state = expect_context::<AppState>();
    let explored = Explored::get();
    let total = EXPERIENCES.len();

    view! {
        <aside
            class="explore-tracker"
            class:complete=move || explored.all_seen()
            class:hidden=move || state.menu_open.get() || state.tour_visible.get()
            aria-label="Your progress through my work"
        >
            <Show
                when=move || explored.all_seen()
                fallback=move || {
                    view! {
                        <span class="tracker-count">
                            {TRACKER_LABEL} " " <strong>{move || explored.count()}</strong> "/" {total}
                        </span>
                        <span class="tracker-bar" aria-hidden="true">
                            <span style:width=move || {
                                format!("{}%", explored.count() * 100 / total)
                            }></span>
                        </span>
                        <ul class="tracker-steps">
                            {EXPERIENCES
                                .iter()
                                .enumerate()
                                .map(|(i, experience)| {
                                    view! {
                                        <li>
                                            <A
                                                href=experience.route
                                                attr:class=move || {
                                                    if explored.is_seen(i) { "tracker-step seen" } else { "tracker-step" }
                                                }
                                                attr:aria-label=move || {
                                                    format!(
                                                        "{}: {}",
                                                        experience.title,
                                                        if explored.is_seen(i) { "seen" } else { "not seen yet" },
                                                    )
                                                }
                                            >
                                                <i
                                                    class=move || {
                                                        if explored.is_seen(i) { "fa-solid fa-check" } else { experience.icon }
                                                    }
                                                    aria-hidden="true"
                                                ></i>
                                                <span>{experience.short}</span>
                                            </A>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                }
            >
                <a class="tracker-cta" href=BOOK_MEETING_URL target="_blank" rel="noopener noreferrer">
                    <i class="fa-solid fa-check-double" aria-hidden="true"></i>
                    <span>{TRACKER_DONE}</span>
                    <i class="fa-solid fa-arrow-right" aria-hidden="true"></i>
                </a>
            </Show>
        </aside>
    }
}
