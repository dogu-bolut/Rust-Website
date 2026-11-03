use leptos::{html, prelude::*};
use leptos_meta::{Meta, Title};
use send_wrapper::SendWrapper;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

use crate::components::{Contact, Hero, NextUp, SectionHeading, Showcase, TechStack};
use crate::data::{
    ERP_BLURB, ERP_CORE, ERP_INTRO, ERP_MODULES, ERP_SHOWCASE_CAPTION, ERP_SHOWCASE_TITLE,
    ERP_TECH, JOURNEY_HEADING, JOURNEY_STEPS, META_ERP,
};
use crate::dom;

const VISIBLE_CLASS: &str = "visible";

/// Observes each step, toggling `visible` so the reveal replays on re-entry.
/// With reduced motion, steps are simply shown.
fn reveal_steps_on_scroll(steps: &[web_sys::HtmlLiElement]) {
    if dom::prefers_reduced_motion() {
        for step in steps {
            let _ = step.class_list().add_1(VISIBLE_CLASS);
        }
        return;
    }

    let callback = Closure::<dyn FnMut(js_sys::Array)>::new(|entries: js_sys::Array| {
        for entry in entries.iter() {
            let entry: IntersectionObserverEntry = entry.unchecked_into();
            let classes = entry.target().class_list();
            let _ = if entry.is_intersecting() {
                classes.add_1(VISIBLE_CLASS)
            } else {
                classes.remove_1(VISIBLE_CLASS)
            };
        }
    });

    let options = IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from_f64(0.3));
    options.set_root_margin("0px 0px -100px 0px");

    let Ok(observer) =
        IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
    else {
        return;
    };
    for step in steps {
        observer.observe(step);
    }

    // The closure must outlive the observer, so cleanup owns both.
    let keep_alive = SendWrapper::new((observer, callback));
    on_cleanup(move || keep_alive.0.disconnect());
}

#[component]
pub fn ErpPage() -> impl IntoView {
    let step_refs: Vec<NodeRef<html::Li>> = JOURNEY_STEPS.iter().map(|_| NodeRef::new()).collect();
    let observed_refs = step_refs.clone();
    // Refs are tracked because on in-app navigation they may not be set on the first run.
    Effect::new(move |_| {
        let steps: Vec<_> = observed_refs.iter().filter_map(|step| step.get()).collect();
        if steps.len() == observed_refs.len() {
            reveal_steps_on_scroll(&steps);
        }
    });

    view! {
        <Title text="Dogu Bolut — ERP"/>
        <Meta name="description" content=META_ERP/>
        <Hero intro=ERP_INTRO>
            <Showcase
                title=ERP_SHOWCASE_TITLE
                caption=format!("{} modules · {}", ERP_MODULES.len(), ERP_SHOWCASE_CAPTION)
                variant="erp"
            >
                <div class="erp-map">
                    <ul class="erp-modules">
                        {ERP_MODULES
                            .iter()
                            .enumerate()
                            .map(|(i, module)| {
                                view! {
                                    <li class="erp-module" style:--i=i.to_string()>
                                        <i class=module.icon aria-hidden="true"></i>
                                        <span>{module.label}</span>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                    <div class="erp-core" aria-hidden="true">
                        <i class="fa-solid fa-database"></i>
                        <span>{ERP_CORE}</span>
                    </div>
                </div>
            </Showcase>
        </Hero>

        <section id="journey" class="section bg-0">
            <div class="container">
                <SectionHeading text=JOURNEY_HEADING/>
                <ol class="story">
                    {JOURNEY_STEPS
                        .iter()
                        .zip(step_refs)
                        .enumerate()
                        .map(|(i, (step, step_ref))| {
                            view! {
                                <li class="story-step" style:--i=i.to_string() node_ref=step_ref>
                                    {*step}
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </div>
        </section>

        <TechStack groups=ERP_TECH/>
        <NextUp current=2/>
        <Contact blurb=ERP_BLURB/>
    }
}
