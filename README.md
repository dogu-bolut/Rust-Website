# Dogu Bolut — Portfolio (Rust + WebAssembly)

Personal portfolio of Dogu Bolut, Software Developer, rebuilt as a [Leptos](https://leptos.dev) 0.8 client-side app compiled to WebAssembly and deployed as static files on Netlify.

All application code is Rust. The only JavaScript in `dist/` is the loader that Trunk and wasm-bindgen generate. The 7 game privacy-policy pages stay as static HTML so that Google Play's links and crawlers work without WASM.

## Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
```

Or download a prebuilt Trunk from <https://github.com/trunk-rs/trunk/releases>.

**Homebrew users:** if `brew install rust` put `cargo` ahead of rustup on your `PATH`, it can't see the wasm target and ignores `rust-toolchain.toml`. Put the rustup toolchain first:

```sh
export PATH="$(dirname "$(rustup which cargo)"):$PATH"
```

## Develop

```sh
./dev.sh               # http://localhost:8080, sets PATH for you
trunk serve            # same, if rustup's cargo is already first on PATH
```

`trunk serve` doesn't apply `public/_redirects`, so extensionless privacy URLs only work on Netlify or under `netlify dev`:

```sh
trunk build --release
npx netlify-cli dev --offline --dir=dist
```

## Build and check

```sh
trunk build --release  # output in dist/
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## Deploy to Netlify

1. Connect the repository in Netlify. All settings come from `netlify.toml`, so nothing needs entering in the UI.
2. Each push then builds and publishes `dist/`.

Netlify's build image ships `rustup` and `cargo`, and `rust-toolchain.toml` selects stable plus the wasm target. Trunk downloads `wasm-bindgen` and `wasm-opt` itself. Check the Trunk release asset URL in `netlify.toml` at <https://github.com/trunk-rs/trunk/releases> if that step fails.

If Netlify builds are too slow, build locally and deploy with the CLI:

```sh
trunk build --release
netlify deploy --prod --dir=dist
```

Trunk hashes JS and WASM filenames, which is why `netlify.toml` can safely cache them as `immutable`.

## Assets

Assets are already in `public/assets/`. To fetch them again from the live site:

```sh
BASE=https://dogubolut.netlify.app/assets
mkdir -p public/assets/{img,pdf,css,pages/privacy}
for f in logofalan.png web-development.png game_console.png graphics-design.png \
         worksamples1.png worksamples2.png worksamples3.png \
         brickpuzzle.svg whatsinmybag.png corridor.png mynewlife.jpg 3dspaceshooter.png \
         tilevania.jpg bountyhunter.jpg spacedrifter.png terminalhacker.png; do
  curl -fsSL "$BASE/img/$f" -o "public/assets/img/$f"
done
curl -fsSL "$BASE/pdf/Dogu%20Bolut%20CV%20-%20English.pdf" -o "public/assets/pdf/Dogu Bolut CV - English.pdf"
for f in common.css navbar.css privacypolicy.css; do
  curl -fsSL "$BASE/css/$f" -o "public/assets/css/$f"
done
for s in brickpuzzle whatsinmybag mynewlife 3dspaceshooter tilevania bountyhunter spacedrifter; do
  curl -fsSL "$BASE/pages/privacy/$s.html" -o "public/assets/pages/privacy/$s.html"
done
```

Pages fetched this way still contain the old scripts, so apply the edits described under Decisions. Netlify may also inject a comment into served HTML, which is why the committed pages were copied from the original repository instead.

## Project layout

- `src/data.rs`: all content as typed constants
- `src/components/`, `src/pages/`: UI that only renders that data
- `src/dom.rs`: small browser helpers
- `style/`: design system, one file per area
- `public/`: copied as is into `dist/`

## Decisions

**Build**
- **Project root:** the project lives directly in this folder rather than a `dogubolut-rs/` subfolder.
- **CSS loading:** Trunk doesn't bundle local `@import`s, so `index.html` links each stylesheet in order. `main.css` loads last and holds only the global reduced-motion override.
- **`wasm-opt` flags:** `index.html` adds `data-wasm-opt-params="--enable-bulk-memory --enable-nontrapping-float-to-int"`. Trunk's bundled `wasm-opt` otherwise rejects what current Rust emits and the release build fails.
- **`strip = "debuginfo"` instead of `strip = true`:** full stripping removes the `target_features` section and makes wasm-bindgen warn. The cost is under 1 KB.
- **Extra dependency:** `send_wrapper` lets the non-`Send` `IntersectionObserver` and its `Closure` be held by Leptos's `on_cleanup`.

**Privacy pages**
- **Back link:** the original used `<button onclick="...">`, which is inline JS. It is now `<a href="/games" class="btn-back-to-games">`, with the same styling.
- **CSS addition:** `privacypolicy.css` gains a `.privacy-policy a.btn-back-to-games` rule, because the generic `.privacy-policy a` rule would otherwise restyle the link and cut off its bottom border. The pages' other CSS is unchanged.
- **Scripts removed:** this also removes the code that filled `<span id="date">`, so the year no longer appears after the heading. The "Last updated" line is unaffected.

**Content and icons**
- **Icon:** `fab fa-css` exists in Font Awesome 6.7.1, so the `fa-css3-alt` fallback wasn't needed.
- **Tech icons:** Node, Angular and the generic database icon are replaced by Rust (Font Awesome), Claude and PostgreSQL. Font Awesome has no Claude or PostgreSQL icon, so those two are inline SVG paths from Simple Icons 13.21.0 (CC0).
- **Added labels:** the spec left the 404 line and link text open, so they are "The page you are looking for does not exist." and "Back to Home". Page meta descriptions and the "Skip to content" link are also new.
- **Kept as written:** the fourth certificate says "22 September 2020" but its text says "10/22/2020". The original site has the same mismatch.

**Visual cohesion (deliberate departures from the original)**
- **One accent family:** Book Meeting is filled orange-red and Download CV is a pastel-orange outline, replacing the original blue and green. The submit button is a pill like every other CTA.
- **Hero art:** the logo no longer pulses or hue-shifts, and it sits at 60% of a 360px square (280px on phones) with the tech icons on a ring around it, so they never overlap. The floating tech icons share one muted pastel orange and twinkle softly instead of using eight brand colours.
- **Shared width:** every hero illustration is capped by `--hero-visual` (440px).
- **Experience heroes:** the screenshot stack (Web), gameplay video (Games) and tilting illustration (ERP) are replaced by one shared app-window component, `Showcase`. Web shows a browser that cycles through the work samples; clicking it opens the image viewer. Games shows a library grid of all nine covers, each scrolling to its card. ERP shows the eight modules wired to one core, with a signal passing between them. The 42 MB `games-reel.mp4` and `erpwelcome.png` are no longer shipped; the originals are still in the old Portfolio repo.
- **Warmer hero:** the blue glow in the hero background is gone.

**Engagement hook**
- **Progress tracker:** a floating pill (`ExploreTracker`) shows how many of the three experience pages the visitor has seen, with a step for each. Once all three are seen it becomes a "Seen it all? Let's talk" button to the booking link. Progress is kept in `localStorage` (`explored-v1`, in `src/progress.rs`); if storage is blocked, it only lasts for the visit.
- **Next up:** each experience page ends with a card pointing to the next experience not yet seen, or the meeting and CV buttons once all are seen.
- **Home proof:** an availability badge, numbers computed from the data (games, live websites, years in ERP since 2024, certificates), a count chip and a "Seen" marker on each experience card, and a lead line under "Experiences".
- **New copy:** the home description, the availability line and the tracker and Next up wording are new, not from the original site.

**Behaviour**
- **Navbar height:** `<header id="home">` keeps its height when the bar goes fixed, so the page doesn't jump.
- **Meta description:** the static tag in `index.html` serves crawlers without WASM. The app removes it on start so each page's `<Meta>` is the only one.
- **Reduced motion:** the Web screenshots don't auto-cycle and the ERP signal doesn't run. Smooth scrolling becomes instant.
- **Keyboard support:** added a skip link, focus moving into the side menu and modal, focus returning when they close, and a Tab trap in the modal.
- **Thumbnails:** the browser screen and its three tabs are `<button>`s, so the keyboard can switch and open samples. Auto-cycling pauses on hover or focus.
- **Icon placement:** floating icons use percentage offsets from the image, not the original's fixed pixels, so they hold up at any size.
- **Unused assets:** images the new site doesn't reference, and the old `assets/js`, were not copied.
