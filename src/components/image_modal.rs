use leptos::{html, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

use crate::data::WorkSample;
use crate::dom;

const MODAL_ID: &str = "image-modal";

/// Id of the thumbnail that opens image `index`, used to restore focus on close.
pub fn trigger_id(index: usize) -> String {
    format!("work-sample-{index}")
}

fn previous(index: usize, len: usize) -> usize {
    (index + len - 1) % len
}

fn next(index: usize, len: usize) -> usize {
    (index + 1) % len
}

/// Keeps Tab inside the modal's buttons.
fn trap_focus(ev: &KeyboardEvent, modal: &Element) {
    let Ok(buttons) = modal.query_selector_all("button") else {
        return;
    };
    let count = buttons.length();
    if count == 0 {
        return;
    }
    let active = document().active_element();
    let current = (0..count).find(|i| {
        buttons
            .item(*i)
            .zip(active.as_ref())
            .is_some_and(|(button, active)| button == **active)
    });
    let target = match (current, ev.shift_key()) {
        (Some(0) | None, true) => count - 1,
        (Some(i), true) => i - 1,
        (Some(i), false) => (i + 1) % count,
        (None, false) => 0,
    };
    ev.prevent_default();
    if let Some(button) = buttons
        .item(target)
        .and_then(|node| node.dyn_into::<HtmlElement>().ok())
    {
        let _ = button.focus();
    }
}

#[component]
pub fn ImageModal(images: &'static [WorkSample], index: RwSignal<Option<usize>>) -> impl IntoView {
    let len = images.len();
    let close_button = NodeRef::<html::Button>::new();
    let modal = NodeRef::<html::Div>::new();

    let go_previous = move || index.update(|i| *i = i.map(|i| previous(i, len)));
    let go_next = move || index.update(|i| *i = i.map(|i| next(i, len)));

    let handle = window_event_listener(leptos::ev::keydown, move |ev| {
        if index.get_untracked().is_none() {
            return;
        }
        match ev.key().as_str() {
            "Escape" => index.set(None),
            "ArrowLeft" => go_previous(),
            "ArrowRight" => go_next(),
            "Tab" => {
                if let Some(modal) = modal.get_untracked() {
                    trap_focus(&ev, &modal);
                }
            }
            _ => {}
        }
    });
    on_cleanup(move || {
        handle.remove();
        dom::set_body_scroll_locked(false);
    });

    // Lock scrolling while open and return focus to the thumbnail on close.
    Effect::new(move |previous_index: Option<Option<usize>>| {
        let current = index.get();
        dom::set_body_scroll_locked(current.is_some());
        if let (Some(Some(was_open)), None) = (previous_index, current) {
            dom::focus_id(&trigger_id(was_open));
        }
        current
    });

    // Move focus into the modal once its close button exists.
    Effect::new(move |_| {
        if let Some(button) = close_button.get() {
            let _ = button.focus();
        }
    });

    view! {
        <Show when=move || index.get().is_some()>
            <div
                id=MODAL_ID
                class="image-modal"
                role="dialog"
                aria-modal="true"
                aria-label="Work samples gallery"
                node_ref=modal
                on:click=move |ev| {
                    let on_backdrop = ev
                        .target()
                        .and_then(|target| target.dyn_into::<Element>().ok())
                        .is_some_and(|target| target.id() == MODAL_ID);
                    if on_backdrop {
                        index.set(None);
                    }
                }
            >
                <button
                    class="modal-close"
                    type="button"
                    aria-label="Close image"
                    node_ref=close_button
                    on:click=move |_| index.set(None)
                >
                    "×"
                </button>
                <button
                    class="modal-arrow modal-prev"
                    type="button"
                    aria-label="Previous image"
                    on:click=move |_| go_previous()
                >
                    "❮"
                </button>
                <figure class="modal-figure">
                    <img
                        class="modal-image"
                        src=move || index.get().map_or("", |i| images[i].src)
                        alt=move || index.get().map_or("", |i| images[i].alt)
                    />
                    <figcaption class="modal-caption">
                        {move || index.get().map_or("", |i| images[i].alt)}
                    </figcaption>
                </figure>
                <button
                    class="modal-arrow modal-next"
                    type="button"
                    aria-label="Next image"
                    on:click=move |_| go_next()
                >
                    "❯"
                </button>
            </div>
        </Show>
    }
}
