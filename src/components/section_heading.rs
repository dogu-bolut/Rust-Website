use leptos::prelude::*;

#[component]
pub fn SectionHeading(text: &'static str) -> impl IntoView {
    view! {
        <div class="section-heading">
            <h2>{text}</h2>
        </div>
    }
}
