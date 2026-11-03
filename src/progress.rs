//! Which experience pages this visitor has seen, remembered in `localStorage`
//! so the tour survives reloads. Storage can be unavailable (private mode,
//! blocked site data); the tracker then simply starts empty each visit.

use leptos::prelude::*;

use crate::data::{Experience, EXPERIENCES};

const STORAGE_KEY: &str = "explored-v1";

#[derive(Clone, Copy)]
pub struct Explored(RwSignal<Vec<bool>>);

impl Explored {
    pub fn provide() -> Self {
        let explored = Self(RwSignal::new(load()));
        provide_context(explored);
        explored
    }

    pub fn get() -> Self {
        expect_context::<Self>()
    }

    pub fn is_seen(self, index: usize) -> bool {
        self.0
            .with(|seen| seen.get(index).copied().unwrap_or(false))
    }

    pub fn count(self) -> usize {
        self.0.with(|seen| seen.iter().filter(|s| **s).count())
    }

    pub fn all_seen(self) -> bool {
        self.count() == EXPERIENCES.len()
    }

    /// Marks the experience whose route is `pathname`, if any.
    pub fn visit(self, pathname: &str) {
        let Some(index) = EXPERIENCES.iter().position(|e| e.route == pathname) else {
            return;
        };
        if self.is_seen(index) {
            return;
        }
        self.0.update(|seen| seen[index] = true);
        self.0.with_untracked(|seen| save(seen));
    }

    /// The first experience not yet seen, from the start.
    pub fn first_unseen(self) -> Option<(usize, &'static Experience)> {
        (0..EXPERIENCES.len())
            .find(|&i| !self.is_seen(i))
            .map(|i| (i, &EXPERIENCES[i]))
    }

    /// The first experience after `current` (wrapping) not yet seen.
    pub fn next_unseen(self, current: usize) -> Option<(usize, &'static Experience)> {
        let len = EXPERIENCES.len();
        (1..len)
            .map(|step| (current + step) % len)
            .find(|&i| !self.is_seen(i))
            .map(|i| (i, &EXPERIENCES[i]))
    }
}

fn storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

fn load() -> Vec<bool> {
    let stored = storage()
        .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
        .unwrap_or_default();
    let bits: Vec<bool> = stored.chars().map(|c| c == '1').collect();
    (0..EXPERIENCES.len())
        .map(|i| bits.get(i).copied().unwrap_or(false))
        .collect()
}

fn save(seen: &[bool]) {
    let value: String = seen.iter().map(|s| if *s { '1' } else { '0' }).collect();
    if let Some(storage) = storage() {
        let _ = storage.set_item(STORAGE_KEY, &value);
    }
}
