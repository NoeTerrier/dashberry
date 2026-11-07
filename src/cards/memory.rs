use std::time::Duration;

use dioxus::prelude::*;
use dioxus_icons::lucide::MemoryStick;

use crate::{
    backend::memory_info,
    cards::common::percent,
    components::{
        card::{Card, CardContent, CardHeader, CardTitle},
        progress::Progress,
    },
};

#[component]
pub fn MemoryCard() -> Element {
    let mut memory = use_signal(|| None);

    use_future(move || async move {
        loop {
            memory.set(Some(memory_info().await.unwrap()));
            gloo_timers::future::sleep(Duration::from_secs(3)).await
        }
    });

    rsx! {
        Card {
            CardHeader {
                CardTitle { MemoryStick {} "Memory" }
            }
            CardContent {
                match &*memory.read() {
                    Some(m) if m.total_memory > 0 => {
                        let used_memory = m.used_memory as f64 / 10u64.pow(9) as f64;
                        let total_memory = m.total_memory as f64 / 10u64.pow(9) as f64;
                        let percent_memory = percent(m.used_memory, m.total_memory);

                        let used_swap = m.used_swap as f64 / 10u64.pow(9) as f64;
                        let total_swap = m.total_swap as f64 / 10u64.pow(9) as f64;
                        let percent_swap = percent(m.used_swap, m.total_swap);

                        rsx! {
                            div {
                                class: "memory",
                                span { "RAM" }
                                Progress { aria_label: "Disk usage", value: used_memory, max: total_memory, style: "margin-top: 0.5em; margin-bottom: 0.5em;" }
                                span { class: "note", "{used_memory:.2} / {total_memory:.2} Go ({percent_memory:.2}%)" }
                            }
                            div {
                                class: "memory",
                                span { "SWAP" }
                                Progress { aria_label: "Disk usage", value: used_swap, max: total_swap, style: "margin-top: 0.5em; margin-bottom: 0.5em;" }
                                span { class: "note", "{used_swap:.2} / {total_swap:.2} Go ({percent_swap:.2}%)" }
                            }
                        }
                    },
                    _ => rsx! { p { "Loading..." } }
                }
            }
        }
    }
}
