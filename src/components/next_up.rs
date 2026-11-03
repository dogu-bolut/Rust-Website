use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::CtaButtons;
use crate::data::{
    EXPERIENCES, NEXT_UP_DONE_TEXT, NEXT_UP_DONE_TITLE, NEXT_UP_KICKER, TRACKER_LABEL,
};
use crate::progress::Explored;

/// End-of-page bridge on an experience page: points to the next experience
/// not yet seen, or, once all are seen, asks for the meeting.
#[component]
pub fn NextUp(
    /// Index of the current page in `EXPERIENCES`.
    current: usize,
) -> impl IntoView {
    let explored = Explored::get();
    let total = EXPERIENCES.len();

    view! {
        <section class="next-up section bg-1">
            <div class="container">
                {move || match explored.next_unseen(current) {
                    Some((_, next)) => {
                        view! {
                            <A href=next.route attr:class="next-up-card">
                                <div class="next-up-text">
                                    <span class="next-up-kicker">
                                        {NEXT_UP_KICKER} " · " {TRACKER_LABEL} " " {explored.count()} "/"
                                        {total}
                                    </span>
                                    <h2>"Next up: " {next.title}</h2>
                                    <p>{next.description}</p>
                                </div>
                                <span class="next-up-arrow" aria-hidden="true">
                                    <i class=next.icon></i>
                                    <i class="fa-solid fa-arrow-right"></i>
                                </span>
                            </A>
                        }
                            .into_any()
                    }
                    None => {
                        view! {
                            <div class="next-up-card done">
                                <div class="next-up-text">
                                    <span class="next-up-kicker">
                                        <i class="fa-solid fa-check-double" aria-hidden="true"></i>
                                        " " {TRACKER_LABEL} " " {total} "/" {total}
                                    </span>
                                    <h2>{NEXT_UP_DONE_TITLE}</h2>
                                    <p>{NEXT_UP_DONE_TEXT}</p>
                                </div>
                                <div class="next-up-actions">
                                    <CtaButtons/>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </section>
    }
}
