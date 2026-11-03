use leptos::prelude::*;

/// The shared hero visual: one app-window frame used on every experience page,
/// so the three pages differ only in what the window shows.
#[component]
pub fn Showcase(
    /// Text in the title bar (a URL for the browser variant, a label otherwise).
    title: &'static str,
    /// Short summary line under the window.
    caption: String,
    /// Extra class for page-specific styling of the body.
    variant: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <figure class=format!("showcase showcase-{variant}")>
            <div class="showcase-window">
                <div class="showcase-bar" aria-hidden="true">
                    <span class="showcase-dots">
                        <span></span>
                        <span></span>
                        <span></span>
                    </span>
                    <span class="showcase-title">{title}</span>
                </div>
                <div class="showcase-body">{children()}</div>
            </div>
            <figcaption class="showcase-caption">{caption}</figcaption>
        </figure>
    }
}
