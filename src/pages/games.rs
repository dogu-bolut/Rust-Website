use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{Contact, Hero, NextUp, SectionHeading, Showcase, TechStack};
use crate::data::{
    GAMES, GAMES_BLURB, GAMES_HEADING, GAMES_HERO_LINKS, GAMES_INTRO, GAMES_SHOWCASE_CAPTION,
    GAMES_SHOWCASE_TITLE, GAMES_TECH, LEGAL_INFO, META_GAMES,
};
use crate::dom;

fn game_id(index: usize) -> String {
    format!("game-{}", index + 1)
}

#[component]
pub fn GamesPage() -> impl IntoView {
    view! {
        <Title text="Dogu Bolut — Games"/>
        <Meta name="description" content=META_GAMES/>
        <Hero intro=GAMES_INTRO socials=GAMES_HERO_LINKS>
            <Showcase
                title=GAMES_SHOWCASE_TITLE
                caption=format!("{} games · {}", GAMES.len(), GAMES_SHOWCASE_CAPTION)
                variant="library"
            >
                <ul class="library-grid">
                    {GAMES
                        .iter()
                        .enumerate()
                        .map(|(i, game)| {
                            let id = game_id(i);
                            view! {
                                <li style:--i=i.to_string()>
                                    <a
                                        class="library-tile"
                                        href=format!("#{id}")
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            dom::scroll_to_id(&id);
                                        }
                                    >
                                        <img src=game.image alt=""/>
                                        <span class="library-name">{game.name}</span>
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </Showcase>
        </Hero>

        <section id="games" class="section bg-0">
            <div class="container">
                <SectionHeading text=GAMES_HEADING/>
                <div class="card-grid">
                    {GAMES
                        .iter()
                        .enumerate()
                        .map(|(i, game)| {
                            view! {
                                <article class="game-card" id=game_id(i)>
                                    <img src=game.image alt=game.name loading="lazy"/>
                                    <div class="game-body">
                                        <h2>
                                            <a href=game.title_url target="_blank" rel="noopener noreferrer">
                                                {game.name}
                                            </a>
                                        </h2>
                                        <p>{game.description}</p>
                                        <div class="pill-row">
                                            {game
                                                .links
                                                .iter()
                                                .map(|link| {
                                                    view! {
                                                        <a
                                                            class="link-pill"
                                                            href=link.url
                                                            target="_blank"
                                                            rel="noopener noreferrer"
                                                        >
                                                            {link.label}
                                                            <i class="fa fa-arrow-right" aria-hidden="true"></i>
                                                        </a>
                                                    }
                                                })
                                                .collect_view()}
                                            {game
                                                .privacy
                                                .map(|path| {
                                                    view! {
                                                        <a class="link-pill" href=path rel="external">
                                                            {LEGAL_INFO}
                                                            <i class="fa fa-arrow-right" aria-hidden="true"></i>
                                                        </a>
                                                    }
                                                })}
                                        </div>
                                    </div>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>

        <TechStack groups=GAMES_TECH/>
        <NextUp current=1/>
        <Contact blurb=GAMES_BLURB/>
    }
}
