use leptos::{html, prelude::*};
use leptos_router::components::A;
use send_wrapper::SendWrapper;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{IntersectionObserver, IntersectionObserverEntry};

use crate::app::AppState;

use crate::data::{
    experience_stat, BOOK_MEETING_URL, EXPERIENCES, TOUR_CONTINUE, TOUR_DONE, TOUR_KICKER,
    TOUR_START, TOUR_TITLE,
};
use crate::progress::Explored;

/// Keeps `visible` in step with whether `card` is on screen.
fn track_visibility(card: &web_sys::HtmlDivElement, visible: RwSignal<bool>) {
    let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
        if let Some(entry) = entries.iter().last() {
            let entry: IntersectionObserverEntry = entry.unchecked_into();
            visible.set(entry.is_intersecting());
        }
    });
    let Ok(observer) = IntersectionObserver::new(callback.as_ref().unchecked_ref()) else {
        return;
    };
    observer.observe(card);
    // The closure must outlive the observer, so cleanup owns both.
    let keep_alive = SendWrapper::new((observer, callback));
    on_cleanup(move || keep_alive.0.disconnect());
}

/// Home hero hook: the three experiences as a numbered tour, with one
/// button that always points at the next thing worth seeing.
#[component]
pub fn TourCard() -> impl IntoView {
    let explored = Explored::get();
    let total = EXPERIENCES.len();
    let next_index = move || explored.first_unseen().map(|(i, _)| i);

    let state = expect_context::<AppState>();
    let card = NodeRef::<html::Div>::new();
    Effect::new(move |_| {
        if let Some(card) = card.get() {
            track_visibility(&card, state.tour_visible);
        }
    });
    on_cleanup(move || state.tour_visible.set(false));

    view! {
        <div node_ref=card class="tour-card" class:complete=move || explored.all_seen()>
            <div class="tour-head">
                <span class="tour-kicker">
                    <i class="fa-solid fa-route" aria-hidden="true"></i>
                    " "
                    {TOUR_KICKER}
                </span>
                <span class="tour-count">
                    <strong>{move || explored.count()}</strong>
                    "/"
                    {total}
                    " seen"
                </span>
            </div>
            <p class="tour-title">{TOUR_TITLE}</p>
            <ol class="tour-steps">
                {EXPERIENCES
                    .iter()
                    .enumerate()
                    .map(|(i, experience)| {
                        view! {
                            <li>
                                <A
                                    href=experience.route
                                    attr:class=move || {
                                        if explored.is_seen(i) {
                                            "tour-step seen"
                                        } else if next_index() == Some(i) {
                                            "tour-step next"
                                        } else {
                                            "tour-step"
                                        }
                                    }
                                >
                                    <span class="tour-step-num" aria-hidden="true">
                                        {move || {
                                            if explored.is_seen(i) {
                                                view! { <i class="fa-solid fa-check"></i> }.into_any()
                                            } else {
                                                view! { <i class=experience.icon></i> }.into_any()
                                            }
                                        }}
                                    </span>
                                    <span class="tour-step-text">
                                        <strong>{experience.short}</strong>
                                        <small>{experience_stat(experience.route)}</small>
                                    </span>
                                </A>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
            {move || match explored.first_unseen() {
                Some((_, next)) => {
                    let label = if explored.count() == 0 {
                        TOUR_START.to_string()
                    } else {
                        format!("{TOUR_CONTINUE} {}", next.short)
                    };
                    view! {
                        <A href=next.route attr:class="tour-go">
                            {label}
                            <i class="fa-solid fa-arrow-right" aria-hidden="true"></i>
                        </A>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <a class="tour-go" href=BOOK_MEETING_URL target="_blank" rel="noopener noreferrer">
                            {TOUR_DONE}
                            <i class="fa-solid fa-arrow-right" aria-hidden="true"></i>
                        </a>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
