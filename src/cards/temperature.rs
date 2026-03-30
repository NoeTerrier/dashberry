use std::time::Duration;

use dioxus::prelude::*;

use crate::{backend::temperature_info, cards::common::CardTitle};

#[component]
pub fn TemperatureCard() -> Element {
    let mut temperature = use_signal(|| 0f32);

    use_future(move || async move {
        loop {
            temperature.set(temperature_info().await.unwrap());
            gloo_timers::future::sleep(Duration::from_secs(3)).await
        }
    });

    rsx! {
        div {
            class: "card",
            CardTitle { title: "🌡 Temperature" }
            div {
                class: "temperature-div",
                span { class: "temperature-value", "{temperature:.2}" }
                span { "°C" }
            }
            p { class: "temperature-hint",
                if *temperature.read() > 50f32 {
                    "🔥 warm"
                } else {
                    "🌀 ideal"
                }
            }
        }
    }
}
