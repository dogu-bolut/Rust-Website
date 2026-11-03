use leptos::prelude::*;

use crate::components::SectionHeading;
use crate::data::{TechGroup, TECH_STACK_HEADING};

#[component]
pub fn TechStack(groups: &'static [TechGroup]) -> impl IntoView {
    view! {
        <section id="tech-stack" class="section bg-1">
            <div class="container">
                <SectionHeading text=TECH_STACK_HEADING/>
                <div class="tech-grid">
                    {groups
                        .iter()
                        .map(|group| {
                            view! {
                                <div class="tech-group">
                                    <h3>{group.heading}</h3>
                                    <ul class="chips">
                                        {group
                                            .items
                                            .iter()
                                            .map(|item| view! { <li class="chip">{*item}</li> })
                                            .collect_view()}
                                    </ul>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
