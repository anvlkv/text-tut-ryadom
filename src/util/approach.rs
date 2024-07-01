use dioxus_hooks::use_context;
use dioxus_signals::Readable;

use crate::state::StateCtx;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Approach {
    Polite,
    Familiar,
}

impl Approach {
    pub fn key(self, key: &'static str) -> String {
        match self {
            Approach::Polite => format!("{key}.polite"),
            Approach::Familiar => format!("{key}.familiar"),
        }
    }
}

pub fn use_approach() -> dioxus_signals::MappedSignal<Approach> {
    let StateCtx(state, _) = use_context();

    state.map(|s| {
        s.config
            .as_ref()
            .map(|c| c.approach.as_ref())
            .flatten()
            .unwrap_or(&Approach::Polite)
    })
}
