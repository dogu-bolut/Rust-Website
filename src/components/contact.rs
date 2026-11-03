use leptos::prelude::*;

use crate::components::SectionHeading;
use crate::data::{
    CONTACT_HEADING, CONTACT_ITEMS, CONTACT_SUBHEADING, FORM_ACTION, FORM_EMAIL_PLACEHOLDER,
    FORM_MESSAGE_PLACEHOLDER, FORM_NAME_PLACEHOLDER, FORM_SUBMIT,
};

#[component]
pub fn Contact(blurb: &'static str) -> impl IntoView {
    view! {
        <section id="contact" class="section bg-0">
            <div class="container">
                <SectionHeading text=CONTACT_HEADING/>
                <div class="contact-grid">
                    <div class="contact-info">
                        <h3>{CONTACT_SUBHEADING}</h3>
                        <p>{blurb}</p>
                        <ul class="contact-items">
                            {CONTACT_ITEMS
                                .iter()
                                .map(|item| {
                                    view! {
                                        <li>
                                            <a
                                                href=item.url
                                                target=item.new_tab.then_some("_blank")
                                                rel=item.new_tab.then_some("noopener noreferrer")
                                            >
                                                <i class=item.icon aria-hidden="true"></i>
                                                <span>{item.label}</span>
                                            </a>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </div>
                    <form class="contact-form" action=FORM_ACTION method="POST">
                        <input
                            type="text"
                            name="name"
                            placeholder=FORM_NAME_PLACEHOLDER
                            aria-label=FORM_NAME_PLACEHOLDER
                            required=true
                        />
                        <input
                            type="email"
                            name="email"
                            placeholder=FORM_EMAIL_PLACEHOLDER
                            aria-label=FORM_EMAIL_PLACEHOLDER
                            required=true
                        />
                        <textarea
                            name="message"
                            rows="6"
                            placeholder=FORM_MESSAGE_PLACEHOLDER
                            aria-label=FORM_MESSAGE_PLACEHOLDER
                            required=true
                        ></textarea>
                        <button type="submit" class="btn-submit">{FORM_SUBMIT}</button>
                    </form>
                </div>
            </div>
        </section>
    }
}
