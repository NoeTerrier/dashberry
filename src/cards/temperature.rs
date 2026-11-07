use std::time::Duration;

use dioxus::prelude::*;
use dioxus_icons::lucide::Thermometer;

use crate::{
    backend::temperature_info,
    components::card::{Card, CardContent, CardHeader, CardTitle},
};

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
        Card {
            CardHeader {
                CardTitle { Thermometer {} "Temperature" }
            }
            CardContent {
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
}
