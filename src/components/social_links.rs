use leptos::prelude::*;

use crate::data::SocialLink;

#[component]
pub fn SocialLinks(links: &'static [SocialLink]) -> impl IntoView {
    view! {
        <div class="social-media">
            {links
                .iter()
                .map(|link| {
                    view! {
                        <a
                            href=link.url
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label=link.label
                        >
                            <i class=link.icon aria-hidden="true"></i>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}
