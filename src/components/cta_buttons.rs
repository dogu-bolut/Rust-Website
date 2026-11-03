use leptos::prelude::*;

use crate::data::{BOOK_MEETING_LABEL, BOOK_MEETING_URL, DOWNLOAD_CV_LABEL, DOWNLOAD_CV_URL};

#[component]
pub fn CtaButtons() -> impl IntoView {
    view! {
        <a
            class="btn-pill btn-meeting"
            href=BOOK_MEETING_URL
            target="_blank"
            rel="noopener noreferrer"
        >
            {BOOK_MEETING_LABEL}
        </a>
        <a class="btn-pill btn-cv" href=DOWNLOAD_CV_URL rel="external" download="">
            {DOWNLOAD_CV_LABEL}
        </a>
    }
}
