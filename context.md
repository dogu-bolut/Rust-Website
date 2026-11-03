# context.md — Dogu Bolut Portfolio, Rebuilt in Rust

> ## ⚡ SINGLE-SHOT BUILD INSTRUCTION — READ FIRST
> You are a senior Rust + WebAssembly engineer. Read this **entire** file, then build the **complete, working project in ONE response**.
> - Execute **Phase 1 → Phase 9 in order, all at once**. Do not stop between phases, do not ask questions, do not wait for approval, do not output a plan and stop.
> - Output **every file in full**. No placeholders, no `// ...rest of code`, no `todo!()`, no "implement similarly for the other pages".
> - Where this file leaves a detail open, make the most reasonable choice, record it in the README's **Decisions** section, and keep going.
> - The finished output must build with `trunk build --release` and deploy to Netlify without manual fixes.

---

## 1. Mission

Recreate **https://dogubolut.netlify.app** — the personal portfolio of **Dogu Bolut**, Software Developer — as a **Rust/WebAssembly single-page application**, deployed as static files on **Netlify**.

Keep the same content, the same URLs, the same dark-and-orange look, and the same interactions. Improve accessibility and performance where it's free to do so. Do not invent new content.

---

## 2. Hard Rules

1. **All application code is Rust.** Routing, components, state, event handling and animations-triggering logic are written in Rust and compiled to WASM with **Leptos**.
2. **No hand-written JavaScript.** No `.js` files in the repo, no inline `<script>` code, no `#[wasm_bindgen(inline_js = ...)]`, no `js_sys::eval`, no JS libraries (no Prism, no jQuery, no Font Awesome JS kit). The only JS in `dist/` is the loader Trunk/wasm-bindgen generate automatically.
3. **Allowed non-Rust files** (configuration, markup shell, styling, content — not program logic): `index.html` (Trunk entry shell), `*.css`, `Cargo.toml`, `Trunk.toml`, `rust-toolchain.toml`, `netlify.toml`, `_redirects`, `README.md`, and static assets (images, video, PDF).
4. **One exception — legal pages:** the 7 game privacy-policy pages stay as **static HTML** copied from the current site (Phase 7). Google Play links to those exact URLs and they must render for crawlers without WASM.
5. **Third-party CSS only:** Google Fonts (Montserrat) and Font Awesome **6.7.1** CSS from cdnjs.
6. **Never change** content text, external URLs, or the contact form endpoint. Only fix accessibility gaps (alt text, aria-labels, focus states).
7. `cargo clippy --target wasm32-unknown-unknown -- -D warnings` must pass. `cargo fmt` applied.

---

## 3. Tech Stack

| Piece | Choice |
|---|---|
| Language | Rust stable, edition 2021, target `wasm32-unknown-unknown` |
| UI framework | `leptos = { version = "0.8", features = ["csr"] }` |
| Routing | `leptos_router = "0.8"` |
| Head / meta tags | `leptos_meta = "0.8"` |
| Browser APIs | `web-sys`, `js-sys`, `wasm-bindgen` |
| Panic messages | `console_error_panic_hook` |
| Bundler | **Trunk 0.21.x** (`trunk build --release`) |
| Hosting | Netlify — static files, SPA fallback |
| Contact form | **Formspree**, existing endpoint, plain HTML `POST` (no JS needed) |

Stay on Leptos **0.8.x**. Older tutorials (0.5/0.6) use different APIs — don't mix them.

### 3.1 Leptos 0.8 API reminders (avoid old-version mistakes)

```rust
use leptos::prelude::*;

// signals
let (open, set_open) = signal(false);          // NOT create_signal

// entry point (src/main.rs)
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

// router
use leptos_router::{components::{Router, Routes, Route}, path, hooks::use_location};

view! {
    <Router>
        <Navbar/>
        <main id="main-content">
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/")      view=HomePage/>
                <Route path=path!("/web")   view=WebPage/>
                <Route path=path!("/games") view=GamesPage/>
                <Route path=path!("/erp")   view=ErpPage/>
            </Routes>
        </main>
        <Footer/>
    </Router>
}

// global listeners (page-scoped ones must be removed on cleanup)
let handle = window_event_listener(leptos::ev::keydown, move |ev| { /* ... */ });
on_cleanup(move || handle.remove());

// meta
provide_meta_context();                         // once, in App
view! { <Title text="Dogu Bolut — Games"/> <Meta name="description" content="..."/> }
```

### 3.2 `Cargo.toml` baseline

```toml
[package]
name = "dogubolut-portfolio"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos_router = "0.8"
leptos_meta = "0.8"
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
  "Window", "Document", "Element", "HtmlElement", "HtmlVideoElement",
  "DomRect", "MouseEvent", "KeyboardEvent",
  "ScrollIntoViewOptions", "ScrollBehavior", "ScrollLogicalPosition", "ScrollToOptions",
  "IntersectionObserver", "IntersectionObserverEntry", "IntersectionObserverInit",
  "CssStyleDeclaration", "MediaQueryList"
] }

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

## 4. Project Structure

```
dogubolut-rs/
├── Cargo.toml
├── rust-toolchain.toml          # channel = "stable", targets = ["wasm32-unknown-unknown"]
├── Trunk.toml                   # [build] target="index.html", dist="dist", public_url="/"
├── index.html                   # Trunk shell: fonts, FA css, data-trunk links, <noscript> fallback
├── netlify.toml
├── README.md
├── style/
│   ├── main.css                 # @imports nothing remote; order: tokens → base → components → pages → responsive
│   ├── tokens.css
│   ├── base.css
│   ├── navbar.css
│   ├── hero.css
│   ├── sections.css             # headings, experiences, certificates, tech stack
│   ├── cards.css                # project cards, game cards, link pills
│   ├── modal.css
│   ├── contact.css
│   ├── footer.css
│   ├── animations.css
│   └── responsive.css
├── public/                      # copied verbatim into dist/
│   ├── _redirects
│   └── assets/
│       ├── img/ …               # see §5.8 asset manifest
│       ├── videos/games-reel.mp4
│       ├── pdf/Dogu Bolut CV - English.pdf
│       ├── css/                 # legacy CSS used ONLY by privacy pages
│       │   ├── common.css
│       │   ├── navbar.css
│       │   └── privacypolicy.css
│       └── pages/privacy/
│           ├── brickpuzzle.html
│           ├── whatsinmybag.html
│           ├── mynewlife.html
│           ├── 3dspaceshooter.html
│           ├── tilevania.html
│           ├── bountyhunter.html
│           └── spacedrifter.html
└── src/
    ├── main.rs
    ├── app.rs                   # App, Router, scroll-to-top on route change
    ├── data.rs                  # ALL content as typed `const`/`static` data (§5)
    ├── dom.rs                   # small helpers: scroll_to_id, scroll_to_top, current_year, prefers_reduced_motion
    ├── components/
    │   ├── mod.rs
    │   ├── navbar.rs            # top bar + per-route links + CTA buttons + hamburger
    │   ├── side_menu.rs         # mobile drawer + overlay
    │   ├── hero.rs              # shared "welcome-section" layout (left text / right slot)
    │   ├── section_heading.rs
    │   ├── social_links.rs
    │   ├── cta_buttons.rs       # Book Meeting + Download CV
    │   ├── glow_link.rs         # orange pill CTA with cursor-following glow
    │   ├── tech_stack.rs
    │   ├── contact.rs           # contact section, blurb passed as prop
    │   ├── image_modal.rs
    │   ├── back_to_top.rs
    │   └── footer.rs
    └── pages/
        ├── mod.rs
        ├── home.rs
        ├── web.rs
        ├── games.rs
        ├── erp.rs
        └── not_found.rs
```

---

## 5. Content — Single Source of Truth (copy verbatim)

All content lives in `src/data.rs` as typed data (e.g. `pub struct Game { name, description, image, title_url, links: &'static [PillLink], privacy: Option<&'static str> }`). Components only render data — no hard-coded strings inside views except structural labels.

### 5.1 Global

| Key | Value |
|---|---|
| Name | Dogu Bolut (certificate texts use "Doğu Bolut" — keep exactly as written) |
| Role | Software Developer |
| Site title | `Dogu Bolut` |
| Language | `en` |
| Favicon | `/assets/img/logofalan.png` |
| Phone | `+90 536 700 09 31` → `tel:+905367000931` |
| Email | `dogubolut@gmail.com` → `mailto:dogubolut@gmail.com` |
| Location | `Umraniye, Istanbul` → `https://maps.app.goo.gl/HDoekYDTvmD95tMY9` |
| Book Meeting | `https://calendar.app.google/oDQZRGp8SBAZg6HM9` (opens in new tab) |
| Download CV | `/assets/pdf/Dogu Bolut CV - English.pdf` (URL-encode spaces in `href`: `/assets/pdf/Dogu%20Bolut%20CV%20-%20English.pdf`; add `download` attribute) |
| Footer | `Copyright © {current year} Dogu Bolut's curriculum vitae. All Rights Reserved.` — year computed in Rust via `js_sys::Date` |
| Contact form | `action="https://formspree.io/f/xgvlodny"` `method="POST"` — fields: `name` (text, placeholder "Full Name", required), `email` (email, placeholder "Corporate Email", required), `message` (textarea, placeholder "Your Message", required). Button text: **Submit**. Section heading: **Contacts**. Sub-heading: **Let's Work Together!** |

**Social links** (icon-only; add `aria-label` and `target="_blank" rel="noopener noreferrer"`):

| Label | URL | Font Awesome class |
|---|---|---|
| LinkedIn | https://www.linkedin.com/in/dogu-bolut/ | `fab fa-linkedin` |
| GitHub | https://github.com/dogu-bolut | `fab fa-github` |
| Instagram | https://instagram.com/dogubolut | `fab fa-instagram` |
| X | https://x.com/dogubolut | `fa-brands fa-x-twitter` |

### 5.2 Navbar per route

Logo/brand text **"Portfolio"** always links to `/` (router navigation). Nav links are **in-page anchors** of the *current* page — they scroll smoothly; they never change route. "Home" (`#home`) scrolls to the top of the current page.

| Route | Links (text → anchor) |
|---|---|
| `/` | Home → `#home` · Experiences → `#experiences` · Certificates → `#certificates` · Contacts → `#contact` |
| `/web` | Home → `#home` · Projects → `#web-projects` · Tech Stack → `#tech-stack` · Contact → `#contact` |
| `/games` | Home → `#home` · Games → `#games` · Tech Stack → `#tech-stack` · Contact → `#contact` |
| `/erp` | Home → `#home` · Journey → `#journey` · Tech Stack → `#tech-stack` · Contact → `#contact` |

Right side of navbar (desktop): **Book Meeting** and **Download CV** buttons. On mobile they move into the side menu.

### 5.3 Home page (`/`)

**Hero (welcome-section)**
- Intro (small, grey, h6): `Hi, I'm ` + highlighted span `Dogu Bolut`
- Title (h2): `Software Developer`
- Description: `This website shows my portfolio. Feel free to explore the links below, where you can find details about almost every project I've worked on. If you have any questions, please don't hesitate to reach out to me via email.`
- Social links row (4 icons), then CTA buttons (Book Meeting, Download CV)
- Right side: profile image `/assets/img/logofalan.png` (alt: "Dogu Bolut logo") surrounded by **8 floating, fading tech icons**:

| Position class | Icon | Brand color |
|---|---|---|
| first | `fab fa-css` | `#66309a` |
| second | `fa-brands fa-square-js` | `#f7e025` |
| third | `fab fa-html5` | `#f0652c` |
| fourth | `fa-brands fa-node` | `#3e863d` |
| fifth | `fa-brands fa-angular` | `#c1132d` |
| sixth | `fa-brands fa-unity` | `#c6c6c6` |
| seventh | `fa-solid fa-database` | `#00558b` |
| eighth | `fa-brands fa-linux` | `#ff9800` |

Note: `fab fa-css` may not exist in FA 6.7.1 free; if it renders blank, use `fa-brands fa-css3-alt` and note it in Decisions.

**Experiences** (`#experiences`) — 3 cards, each: image, title (links to route), description, orange **"Learn more →"** glow pill (links to route).

| Title | Description | Image | Route |
|---|---|---|---|
| Web Development | Building responsive modern websites using HTML, CSS, and JavaScript. | `/assets/img/web-development.png` | `/web` |
| Game Development | Creating games in Unity with smooth mechanics and engaging design. | `/assets/img/game_console.png` | `/games` |
| ERP | Implement ERP systems to streamline business processes and efficiency. | `/assets/img/graphics-design.png` | `/erp` |

**Certificates** (`#certificates`) — 4 cards. Left "badge" block: big title, divider line, category, type. Right details: date (h2), divider, certificate kind (h3), description (p).

| Badge title | Category | Type | Date | Kind | Description |
|---|---|---|---|---|---|
| UNITY | GAME DEVELOPMENT | COURSE | 20 January 2023 | Completion Certificate | This certificate verifies that Doğu Bolut successfully completed the course RPG Core Combat Creator: Learn Intermediate Unity C# Coding on 01/20/2023 as taught by Ben Tristem, Rick Davidson, GameDev.tv Team on Udemy. |
| DIGIAGE | GAME DEVELOPMENT | SUMMER CAMP | 1-11 September 2022 | Participation Certificate | "Thank you for participating in the 10-day Game Development Summer Camp, hosted by Bilişim Vadisi and organized by DIGIAGE and Oyunla Gelecek." |
| MAĞARA JAM | GAME DEVELOPMENT | HACKATHON | 18-21 February 2022 | Participation Certificate | I joined the 4th Mağara Jam, creating a game within a short time, and earned a participation certificate. |
| UNITY | GAME DEVELOPMENT | COURSE | 22 September 2020 | Completion Certificate | This certificate verifies that Doğu Bolut successfully completed the course Complete C# Unity Game Developer 2D on 10/22/2020 as taught by GameDev.tv Team, Rick Davidson, Gary Pettie on Udemy. |

**Contact blurb (home):** `I am open to full-time positions and professional collaborations. Contact me to discuss potential work or opportunities.`

### 5.4 Web page (`/web`)

**Hero**
- Intro: `This section ` + highlighted `shows`
- Title: `My Websites`
- Description: `Welcome to the website section of my portfolio. This space features my work in web development, where design meets functionality. I enjoy creating fast, accessible, and visually engaging websites that provide real value to users. Every project is an opportunity to turn a concept into a polished digital experience.`
- Social row: single icon `fa-solid fa-diamond` → `https://app.netlify.com/teams/dogubolut/projects` (aria-label "Netlify projects", new tab)
- Right side: **"Work Samples"** stage — 3 stacked screenshots that peek over each other (`--peek: 25vh; --gap: 2vh; --radius: 5px; --maxw: 90vw`). Clicking any image opens the **image modal gallery** (§7 Phase 6).
  - `/assets/img/worksamples1.png` alt "Ersatr"
  - `/assets/img/worksamples2.png` alt "Ersatr"
  - `/assets/img/worksamples3.png` alt "Ersatr"

**Projects** (`#web-projects`) — card grid. Each card: small tag label, title (external link), description, **"Go to Website"** button. All external links open in a new tab.

| Tag | Name | Description | URL |
|---|---|---|---|
| B2B | Ersa Foreign Trade | Designed primarily to act as a digital storefront. | https://www.ersatr.com |
| E-Commerce | Sağlık Paketi | Created custom storefront with responsive layouts and intuitive shopping experience. | https://www.saglikpaketi.com.tr |
| E-Commerce | BAW Shop | Modern e-commerce frontend with responsive layouts and interactive product display. | https://dogubolut-baw.netlify.app/ |
| Movie Review | Angular Movie App | Developed Angular app displaying movies with search, filters, and ratings. | https://dogubolut-movieapp.netlify.app/ |
| API | Dog Breeds | Search and explore dog breeds with real-time API integration functionality. | https://dogubolut-dogbreedlist.netlify.app/ |
| Clone | Youtube | Static homepage mockup of YouTube interface showcasing layout and design. | https://dogubolut-youtube.netlify.app/ |
| Entertainment | Rock Paper Scissors | Interactive Rock Paper Scissors game playable directly in the browser. | https://dogubolut-rockpaperscissors.netlify.app/ |
| Clone | Amazon | Amazon clone project demonstrating e-commerce layout, design, and functionality. | https://dogubolut-amazon.netlify.app/ |
| Radio | Lounge FM | A simple radio website where you can listen live anytime. | https://dogubolut-radio.netlify.app/ |

**Tech Stack** (`#tech-stack`)
- Programming: HTML, CSS, JavaScript, TypeScript
- Framework & Library: Angular, Apache Echarts, Tabulator, Leaflet
- Tool & Deployment: Git, Netlify

**Contact blurb (web):** `I’m actively seeking full-time opportunities where I can contribute my skills in web development. If you’re a recruiter or company representative, feel free to reach out directly.`

### 5.5 Games page (`/games`)

**Hero**
- Intro: `This section ` + highlighted `shows`
- Title: `My Games`
- Description: `Welcome to the games section of my portfolio! Here are games I’ve worked on, from small prototypes to complete projects. Each project reflects my passion for game development, creative problem-solving, and technical design. Feel free to explore and play!`
- Social row: `fa-brands fa-google-play` → `https://play.google.com/store/apps/dev?id=7823482579787878315` (aria "Google Play developer page"), `fa-brands fa-itch-io` → `https://itch.io/profile/dogubolut` (aria "itch.io profile")
- Right side: **"Gameplay Preview"** video `/assets/videos/games-reel.mp4` — `autoplay muted loop playsinline preload="metadata"`, rounded corners. Fallback text inside `<video>`: "Your browser does not support the video tag."

**Games** (`#games`) — card grid. Each card: image, title (external link to `title_url`), description, row of **link pills** (label + `fa fa-arrow-right`). "Legal Info" pill is an internal static link (full page load, not router).

| Name | Image | Description | Title URL | Pills |
|---|---|---|---|---|
| Brick Puzzle | `brickpuzzle.svg` | A relaxing 10×10 block puzzle with timeless, strategic fun for all. | https://play.google.com/store/apps/details?id=com.mr.stijket.BrickPuzzle | Mobile → same URL · Legal Info → `/assets/pages/privacy/brickpuzzle` |
| Çantamda Ne Var? | `whatsinmybag.png` | What's in My Bag? was made in 3 days for Mağara Jam #4. | https://ozgurbyrm.itch.io/cantamda-ne-var | PC → https://ozgurbyrm.itch.io/cantamda-ne-var · Mobile → https://play.google.com/store/apps/details?id=com.WinterSkyStudio.antamdaNeVar · Legal Info → `/assets/pages/privacy/whatsinmybag` |
| Corridor | `corridor.png` | Digiage game development summer camp, table no 18 : Corridor | https://dogubolut.itch.io/corridor | PC → same URL |
| My New Life | `mynewlife.jpg` | A heartfelt visual novel about love and tough choices in high school life. | https://dogubolut.itch.io/my-new-life | PC → same URL · Mobile → https://play.google.com/store/apps/details?id=com.mr.stijket.mynewlife · Legal Info → `/assets/pages/privacy/mynewlife` |
| 3D Space Shooter | `3dspaceshooter.png` | Pilot your ship, dodge obstacles, and blast enemies in this fast arcade shooter. | https://play.google.com/store/apps/details?id=com.mr.stijket.SpaceShooter | Mobile → same URL · Legal Info → `/assets/pages/privacy/3dspaceshooter` |
| Tile Vania | `tilevania.jpg` | Pixel-art platformer with traps and puzzles; dare your skills in an adventure. | https://play.google.com/store/apps/details?id=com.mr.stijket.TileVania | Mobile → same URL · Legal Info → `/assets/pages/privacy/tilevania` |
| Bounty Hunter | `bountyhunter.jpg` | Be a bounty hunter, track foes, and earn rewards in this action-packed game. | https://play.google.com/store/apps/details?id=com.mr.stijket.BountyHunter | Mobile → same URL · Legal Info → `/assets/pages/privacy/bountyhunter` |
| Space Drifter | `spacedrifter.png` | Embark on a cosmic adventure, dodge asteroids, and battle space pirates. | https://play.google.com/store/apps/details?id=com.mr.stijket.SpaceDrifter | Mobile → same URL · Legal Info → `/assets/pages/privacy/spacedrifter` |
| Terminal Hacker | `terminalhacker.png` | Crack the code, solve the word puzzles, and find the hidden clues. | https://dogubolut.itch.io/terminal-hacker | PC → same URL |

(All images live in `/assets/img/`. Alt text = game name.)

**Tech Stack** (`#tech-stack`)
- Game Engine: Unity, Unreal Engine, Ren'Py
- Programming: C#, Git
- Design & Art: Photoshop, Gimp, Blender

**Contact blurb (games):** `I create polished, engaging game experiences and thrive in collaborative environments. I’m ready to contribute my skills to projects that value innovation and quality.`

### 5.6 ERP page (`/erp`)

**Hero**
- Intro: `This section ` + highlighted `shows`
- Title: `My ERP Journey`
- Description: `Welcome to the ERP section of my portfolio. This space highlights my journey in enterprise resource planning, where technology connects and streamlines business processes. I enjoy learning how ERP brings structure and efficiency, and I’m focused on growing my skills to contribute to real-world solutions.`
- No social row.
- Right side: image `/assets/img/erpwelcome.png` (alt "ERP illustration") inside a `.erpwelcome` wrapper with **3D mouse-tilt effect** (Phase 6).

**Journey** (`#journey`) — vertical list of `.story-step` items, revealed one by one on scroll (Phase 6):
1. `🚀 Started as an intern at WorkRunner in March 2024`
2. `💡 After 2 months, I stepped into the role of Junior Developer, expanding my skills`
3. `⚡ Gained expertise in ERP solutions, leading small projects end-to-end`
4. `🌟 Ready to take on new adventures and contribute to innovative teams`

**Tech Stack** (`#tech-stack`)
- Core Development: Javascript, SQL
- Libraries & Tools: Apache ECharts, Tabulator, Leaflet, Other Javascript Libraries
- Infrastructure: Redis, MS SQL, REST API

**Contact blurb (erp):** `I’m seeking opportunities to contribute my experience in ERP development, database management, and process optimization within a collaborative team.`

### 5.7 Not Found page

Any unknown route renders a styled 404 inside the normal layout: heading `404 - Page Not Found`, short line, and an orange glow-pill link back to `/`.

### 5.8 Asset manifest

Preferred: copy the `assets/` folder from the existing site's repository. Otherwise download from the live site (tooling only, not part of the app):

| Live URL | Repo path |
|---|---|
| https://dogubolut.netlify.app/assets/img/logofalan.png | `public/assets/img/logofalan.png` |
| …/assets/img/web-development.png | `public/assets/img/web-development.png` |
| …/assets/img/game_console.png | `public/assets/img/game_console.png` |
| …/assets/img/graphics-design.png | `public/assets/img/graphics-design.png` |
| …/assets/img/worksamples1.png, worksamples2.png, worksamples3.png | `public/assets/img/` |
| …/assets/img/erpwelcome.png | `public/assets/img/erpwelcome.png` |
| …/assets/img/brickpuzzle.svg, whatsinmybag.png, corridor.png, mynewlife.jpg, 3dspaceshooter.png, tilevania.jpg, bountyhunter.jpg, spacedrifter.png, terminalhacker.png | `public/assets/img/` |
| …/assets/videos/games-reel.mp4 | `public/assets/videos/games-reel.mp4` |
| …/assets/pdf/Dogu%20Bolut%20CV%20-%20English.pdf | `public/assets/pdf/Dogu Bolut CV - English.pdf` |
| …/assets/pages/privacy/{brickpuzzle,whatsinmybag,mynewlife,3dspaceshooter,tilevania,bountyhunter,spacedrifter}.html | `public/assets/pages/privacy/` |
| …/assets/css/common.css, navbar.css, privacypolicy.css | `public/assets/css/` (privacy pages only) |

Provide this as a `curl` block in the README's "Assets" section so the owner can fetch everything in one paste.

---

## 6. Design System

### 6.1 Tokens (`style/tokens.css`)

```css
:root {
  /* accent palette (from original site) */
  --color-orange-red: #FF4500;      /* primary accent: CTAs, submit, highlights */
  --color-tomato: #FF6347;
  --color-classic-orange: #FFA500;  /* link hover */
  --color-dark-orange: #FF8C00;
  --color-coral: #FF7F50;
  --color-golden-orange: #E59400;
  --color-chocolate-orange: #D2691E;
  --color-pastel-orange: #FFB347;
  --color-muted-sunset: #E6A157;
  --color-bronze-orange: #CC7722;

  /* surfaces */
  --bg-0: #000000;          /* alternating section bg */
  --bg-1: #111111;          /* alternating section bg, header */
  --bg-2: #121212;          /* side menu */
  --card: #1a1a1a;
  --card-border: rgba(255,255,255,0.08);

  /* text */
  --text: #ffffff;
  --text-muted: #b1b1b1;    /* hero intro h6 */
  --text-soft: #8f8f8f;
  --text-dim: #aaaaaa;

  /* buttons */
  --btn-meeting: #007BFF;  --btn-meeting-hover: #0056b3;  --btn-meeting-glow: rgba(0,123,255,0.4);
  --btn-cv: #28a745;       --btn-cv-hover: #218838;       --btn-cv-glow: rgba(40,167,69,0.4);
  --cta-glow: rgba(255,69,0,0.4);

  --radius-sm: 5px;
  --radius-md: 10px;
  --radius-pill: 999px;
  --nav-h: 68px;
  --container: min(1200px, 90vw);
  --font: "Montserrat", sans-serif;
}
```

The site is **dark-only** (it was never light). Set `color-scheme: dark` on `:root` and an explicit `body` background.

### 6.2 Typography
- Montserrat (Google Fonts, weights 100–900), base 16px, body weight 400.
- Hero intro h6: 20px, weight 500, `--text-muted`; highlighted span in `--color-orange-red`.
- Hero title h2: large (clamp ~2.5rem → 4rem), weight 700, white.
- Section headings ("Experiences", "Certificates", "Games", …): centered, weight 700, white, with a short orange underline bar.
- Card titles h2: ~25px, weight 700, white.

### 6.3 Layout
- Sections alternate background: header/hero `#111` → experiences `#000` → certificates `#111` → contact `#000` (subpages follow the same alternation: hero `#111`, main section `#000`, tech stack `#111`, contact `#000`).
- Every section: `scroll-margin-top: calc(var(--nav-h) + 12px)` so anchor scrolling lands below the fixed navbar.
- Hero: two columns (text left, visual right) on desktop; stacks on mobile with the visual below.
- Subtle aurora background on hero: soft radial gradients `rgba(255,180,120,.1)`, `rgba(255,100,150,.1)`, `rgba(120,130,255,.1)`.

### 6.4 Components
- **Navbar:** brand "Portfolio" left, links center, buttons right. Initially static over `#111`. After scrolling past its own height → `.fixed-nav`: `position: fixed; inset: 0 0 auto 0; padding: .8rem 10%; background: rgba(0,0,0,.85); backdrop-filter: blur(10px); box-shadow: 0 4px 20px rgba(0,0,0,.4); z-index: 9999; transition: all .3s ease-in-out`. Link hover color: `--color-classic-orange`.
- **Book Meeting / Download CV:** pill buttons (`999px`), white uppercase text, weight 600. Blue `#007BFF` with `0 6px 20px rgba(0,123,255,.4)` glow; green `#28a745` with `0 6px 20px rgba(40,167,69,.4)` glow. Darker shade on hover + slight lift.
- **Side menu (≤ 992px):** hamburger `fa fa-bars` opens a 300px drawer sliding in from the left (`left: -300px → 0`, `transition: all .5s ease`), `top: var(--nav-h)`, bg `#121212`, contains the route's links and both CTA buttons. A dark overlay (opacity .8) covers the page; main content gets `filter: blur(4px)`. Closes on: link click, overlay click, Escape, route change.
- **Glow pill CTA ("Learn more", 404 link):** `display:inline-flex; gap:10px; padding:12px 20px; background: var(--color-orange-red); color:#fff; font-weight:700; text-transform:uppercase; border-radius:999px; box-shadow: 0 6px 20px var(--cta-glow); position:relative; overflow:hidden`. `::before` = 160px radial white glow (`rgba(255,255,255,.2)`) positioned at `left: var(--x,50%); top: var(--y,50%)` (updated from Rust on mousemove). `::after` = skewed white shine (`rgba(255,255,255,.15)`, `skewX(-20deg)`) sweeping from `left:-100%` to `120%` on hover over .6s.
- **Experience cards:** dark card, orange top indicator bar, image, title, text, glow pill. Hover: lift + orange glow `rgba(255,69,0,.4)`.
- **Certificate cards:** horizontal; left badge block with dark gradient (`#2c2c2c → #0a0a0a`), right details on `#111`; divider lines `#666`; text `#ddd`/`#aaa`. Stack vertically on mobile.
- **Project / game cards:** grid `repeat(auto-fill, minmax(280px, 1fr))`, consistent heights, image with `object-fit: cover`. Project tag = small orange-outlined label. **Link pills** = small rounded buttons with arrow icon.
- **Tech stack:** 3 group cards, each with a heading and the items as chips.
- **Contact:** two columns — left: "Let's Work Together!", page blurb, contact items (icons `fa-solid fa-phone`, `fa-solid fa-envelope`, `fa-solid fa-house`, each a link); right: form with dark inputs and orange **Submit** button (`--radius-md`).
- **Back-to-top:** round button bottom-right with `fas fa-arrow-up`, visible after `scrollY > 500` (`.show-link`), gentle `bounce` animation.
- **Image modal:** full-screen overlay `rgba(0,0,0,.9)`, centered image, caption from alt, `×` close top-right, `❮` / `❯` arrows at sides.

### 6.5 Animations (`style/animations.css`)

```css
@keyframes slideInLeft { from { opacity: 0; transform: translateX(-40px); } to { opacity: 1; transform: translateX(0); } }
@keyframes fade        { 0%, 100% { opacity: 0; } 50% { opacity: 1; } }
@keyframes pulse       { 0%, 100% { transform: scale(1); filter: brightness(1) saturate(1) hue-rotate(0deg); }
                         50%      { transform: scale(1.15); filter: brightness(1.1) saturate(3) hue-rotate(-20deg); } }
@keyframes fadeInUp    { from { opacity: 0; transform: translateY(30px); } to { opacity: 1; transform: translateY(0); } }
@keyframes bounce      { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-6px); } }
```
- Hero left column: `slideInLeft`. Profile image: `pulse`. Floating tech icons: positioned around the profile image, each with `fade` at staggered delays.
- Wrap all motion in `@media (prefers-reduced-motion: no-preference)`; with reduced motion, everything is visible and static.

### 6.6 Responsive
- `≤ 992px`: hide desktop nav links + buttons, show hamburger.
- `≤ 768px`: hero stacks, certificates stack, contact stacks, grids go 1–2 columns.
- `≤ 480px`: 16px side gutters, smaller hero title, no horizontal scroll anywhere.

---

## 7. Build Phases — execute ALL of them now, in order, in this one response

### Phase 1 — Scaffold & toolchain
Create `Cargo.toml` (§3.2), `rust-toolchain.toml`, `Trunk.toml` (`public_url = "/"` is mandatory so `/web`, `/games`, `/erp` load the WASM from root), and `index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Dogu Bolut</title>
  <meta name="description" content="Dogu Bolut — Software Developer. Web development, game development and ERP portfolio." />
  <meta property="og:title" content="Dogu Bolut — Software Developer" />
  <meta property="og:image" content="/assets/img/logofalan.png" />
  <link rel="icon" type="image/png" href="/assets/img/logofalan.png" />
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Montserrat:wght@100..900&display=swap" />
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.1/css/all.min.css" />
  <link data-trunk rel="css" href="style/main.css" />
  <link data-trunk rel="copy-dir" href="public/assets" />
  <link data-trunk rel="copy-file" href="public/_redirects" />
  <link data-trunk rel="rust" data-wasm-opt="z" />
</head>
<body>
  <noscript>
    <!-- Crawler / no-WASM fallback: name, role, contact links, CV link, links to /web /games /erp -->
  </noscript>
</body>
</html>
```
(Fill the `<noscript>` with real content from §5.1. If Trunk can't bundle multiple CSS files via one entry, either list each file with its own `data-trunk rel="css"` link or concatenate into `main.css` — pick one and note it.)

### Phase 2 — Content layer (`src/data.rs`)
Typed structs + `pub const`/`pub static` arrays for: socials, nav configs per route, experiences, certificates, web projects, work samples, games (with pills + privacy slug), journey steps, tech stacks per page, contact blurbs per page, page intros. Every string from §5 lives here, verbatim.

### Phase 3 — Design system (CSS)
Write all CSS files from §6. Use the tokens everywhere — no stray hex values in component CSS except brand icon colors.

### Phase 4 — App shell & routing
- `App`: `provide_meta_context()`, `<Router>`, `Navbar`, `SideMenu`, `<main>`, `Routes` (§3.1), `BackToTop`, `Footer`.
- App-level state via `provide_context`: `menu_open: RwSignal<bool>`, `scrolled: RwSignal<bool>` (scrollY > nav height), `show_top: RwSignal<bool>` (scrollY > 500). One `window_event_listener(ev::scroll, …)` updates both.
- Navbar reads `use_location().pathname` and picks the matching link set from `data.rs` (fallback: home set).
- On every route change: close side menu, scroll to top (`window.scroll_to_with_x_and_y(0.0, 0.0)`), update `<Title>`.
- The header container has `id="home"`.

### Phase 5 — Pages
Implement `HomePage`, `WebPage`, `GamesPage`, `ErpPage`, `NotFound` by composing shared components: `Hero` (intro, highlight, title, description, optional socials, optional CTA buttons, `children` slot for the right side), `SectionHeading`, `TechStack`, `Contact { blurb }`. Each page sets its own `<Title>` (e.g. "Dogu Bolut — Games") and `<Meta name="description">`.

Routing rules for links:
- Internal routes (`/`, `/web`, `/games`, `/erp`) → `leptos_router::components::A`.
- In-page anchors (`#…`) → plain `<a href="#id">` with `on:click` that calls `ev.prevent_default()` and smooth-scrolls via `dom::scroll_to_id(id)` (`#home` → scroll to top). Do **not** rely on the router to handle hash links.
- Privacy pages, CV PDF → plain `<a>` with `rel="external"` so the router doesn't intercept them.
- External sites → plain `<a target="_blank" rel="noopener noreferrer">`.

### Phase 6 — Interactivity in Rust (replaces the old `main.js` / `router.js`)

| Behavior | Rust implementation |
|---|---|
| Fixed navbar on scroll | `scrolled` signal → `class:fixed-nav=move \|\| scrolled.get()` |
| Back-to-top button | `show_top` signal → `class:show-link`; click → smooth scroll to top |
| Smooth anchor scrolling | `Element::scroll_into_view_with_scroll_into_view_options` with `ScrollBehavior::Smooth` (CSS `scroll-margin-top` handles the navbar offset) |
| Side menu | `menu_open` signal toggled by hamburger; overlay click, link click, Escape and route change set it `false`; `class:blurred` on `<main>` |
| Cursor glow on CTA pills | `on:mousemove` → read `MouseEvent::client_x/y` minus `get_bounding_client_rect()` → set `--x` / `--y` in px (via `style:--x=` if your Leptos version supports custom properties in `style:`, otherwise `HtmlElement::style().set_property("--x", …)` on a `NodeRef`) |
| Image modal gallery (`/web`) | `RwSignal<Option<usize>>`; image click opens at index; `❮`/`❯` wrap around (`(i + len - 1) % len`, `(i + 1) % len`); close on `×`, backdrop click (only when target is the backdrop), Escape; ArrowLeft/ArrowRight navigate. Keydown listener registered in the component and removed in `on_cleanup`. Lock body scroll while open. |
| ERP 3D image tilt | `on:mousemove` on `.erpwelcome img`: `rotateX = ((y - cy) / cy) * -15`, `rotateY = ((x - cx) / cx) * 15`, set `transform: rotateX(..deg) rotateY(..deg) scale(1.05)`; `on:mouseleave` → `rotateX(0) rotateY(0) scale(1)`. Parent has `perspective: 1000px`; image has `transition: transform .15s ease`. Disabled for reduced motion. |
| Journey reveal on scroll | `web_sys::IntersectionObserver` with `threshold: 0.3`, `rootMargin: "0px 0px -100px 0px"` observing each `.story-step` (use `NodeRef`s). Intersecting → add class `visible` (`animation: fadeInUp 1s forwards; animation-delay: index * 0.8s` via `style:--i`). Not intersecting → remove class (replays on re-entry). Keep the `Closure` alive for the component's lifetime and `disconnect()` in `on_cleanup`. |
| Footer year | `js_sys::Date::new_0().get_full_year()` |
| Reduced motion | `dom::prefers_reduced_motion()` via `window.match_media("(prefers-reduced-motion: reduce)")`; skip tilt and make journey steps visible immediately |

No `unwrap()` on DOM lookups that can legitimately be `None` — use `if let`/early return.

### Phase 7 — Static assets & legal pages
- Place assets per §5.8.
- Copy the 7 privacy HTML files **verbatim**, with only these edits: remove the `<script src=".../main.js">` and `<script src=".../router.js">` tags (no JS allowed, and they'd throw errors), and make sure the "← Back to Games" link points to `/games`. Keep their `<link>`s to `/assets/css/common.css`, `/assets/css/navbar.css`, `/assets/css/privacypolicy.css` and ship those three files in `public/assets/css/`.
- Both `/assets/pages/privacy/<slug>` and `/assets/pages/privacy/<slug>.html` must return the page (Google Play uses the extensionless form).

### Phase 8 — Netlify deployment

`public/_redirects` (order matters — specific rules first):
```
/assets/pages/privacy/:slug   /assets/pages/privacy/:slug.html   200
/web     /index.html   200
/games   /index.html   200
/erp     /index.html   200
/*       /index.html   200
```
(Netlify serves existing files before applying non-forced rules, so real assets are never rewritten.)

`netlify.toml`:
```toml
[build]
  publish = "dist"
  command = """
    rustup toolchain install stable --profile minimal && \
    rustup target add wasm32-unknown-unknown && \
    mkdir -p .bin && \
    curl -sSL https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C .bin && \
    ./.bin/trunk build --release
  """

[build.environment]
  RUST_LOG = "warn"

[[headers]]
  for = "/*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
    Cache-Control = "public, max-age=31536000, immutable"

[[headers]]
  for = "/*.js"
  [headers.values]
    Cache-Control = "public, max-age=31536000, immutable"

[[headers]]
  for = "/*"
  [headers.values]
    X-Content-Type-Options = "nosniff"
    Referrer-Policy = "strict-origin-when-cross-origin"
```
Notes for the README:
- Netlify's build image ships `rustup` and `cargo`; `rust-toolchain.toml` makes rustup install stable + the wasm target automatically.
- Verify the Trunk release asset name/version on https://github.com/trunk-rs/trunk/releases and update the URL if it changed.
- Trunk downloads `wasm-bindgen` and `wasm-opt` itself during the build.
- Fallback if Netlify builds are slow: build locally with `trunk build --release` and deploy with `netlify deploy --prod --dir=dist`.
- Trunk hashes JS/WASM filenames by default, which is why the long `immutable` cache is safe.

### Phase 9 — Verify (do this before you finish, and fix anything that fails)
1. `cargo fmt --check`, `cargo clippy --target wasm32-unknown-unknown -- -D warnings`, `trunk build --release` → all clean.
2. `find . -name "*.js" -not -path "./dist/*" -not -path "./target/*"` → **no results**.
3. `trunk serve` and check: `/`, `/web`, `/games`, `/erp` load directly (hard refresh) and via in-app links; unknown path shows 404 page.
4. Every anchor link scrolls to the right section below the navbar on every page.
5. Side menu works at 375px width; no horizontal scroll at 320px.
6. Image modal: open, next/prev wrap, Escape, backdrop click, arrow keys.
7. ERP tilt and journey reveal work; with reduced motion they're static and visible.
8. CV downloads from `/assets/pdf/Dogu%20Bolut%20CV%20-%20English.pdf`.
9. All 7 privacy URLs load **with and without** `.html`, styled, with no console errors.
10. Contact form posts to Formspree.
11. Every content string matches §5 exactly; every image has meaningful alt text; every icon-only link has an `aria-label`.

---

## 8. Output Format (what your single response must contain)

1. One-paragraph summary of the architecture.
2. The full file tree.
3. **Every file**, in full, each under a heading with its path, in a fenced code block with the right language tag. Order: config files → `index.html` → `style/*` → `src/*` → `public/_redirects` → `README.md`.
4. For binary assets and the privacy pages, don't print contents — list them and give the exact copy/download commands (README "Assets" section).
5. `README.md` must include: prerequisites (`rustup target add wasm32-unknown-unknown`, install Trunk), `trunk serve` for local dev, `trunk build --release`, Netlify setup (connect repo, settings come from `netlify.toml`), the CLI fallback deploy, the asset fetch commands, and a **Decisions** section listing every judgment call you made.

---

## 9. Definition of Done

- [ ] Pure Rust app code (Leptos 0.8 CSR), zero hand-written JS.
- [ ] Routes `/`, `/web`, `/games`, `/erp` + 404, all deep-linkable on Netlify.
- [ ] All content from §5 present and verbatim; all links correct.
- [ ] Dark + orange design system from §6, Montserrat, Font Awesome icons.
- [ ] Every interaction from Phase 6 implemented in Rust.
- [ ] Privacy pages preserved at their exact URLs.
- [ ] Builds clean; deploys with `netlify.toml` as-is.
- [ ] Accessible: alt text, aria-labels, visible focus states, reduced-motion support, keyboard-usable menu and modal.

**Now build it — all phases, all files, in this one response.**
