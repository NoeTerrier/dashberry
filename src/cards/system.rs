use gloo_timers::future::TimeoutFuture;
use std::time::Duration;

use dioxus::prelude::*;

use crate::{
    backend::system_info,
    cards::common::{CardTitle, InfoType, Row},
};
use humantime::format_duration;

#[component]
pub fn SystemCard() -> Element {
    let mut system = use_signal(|| None);

    use_future(move || async move {
        system.set(Some(system_info().await.unwrap()));
    });

    rsx!(
        div {
            class: "card",
            CardTitle { title: "🖥️ System" }
            {
                match &*system.read_unchecked() {
                    Some(s) => {
                        let mut t = use_signal(|| s.uptime);
                        let mut uptime = use_signal(|| "".to_string());
                        spawn(async move {
                            TimeoutFuture::new(1000).await;
                            t += 1;
                            uptime.set(format_duration(Duration::from_secs(*t.read())).to_string());
                        });
                        rsx! {
                            Row { label: "Hostname", value: &s.hostname, info_type: InfoType::Highlight }
                            Row { label: "Model", value: &s.product_name, info_type: InfoType::Default }
                            Row { label: "OS", value: &s.os, info_type: InfoType::Note }
                            Row { label: "Kernel", value: &s.kernel, info_type: InfoType::Note }
                            Row { label: "Uptime", value: uptime,  info_type: InfoType::Accent }
                            Row { label: "CPU Cores", value: s.cpu_cores,  info_type: InfoType::Default }
                        }
                    },
                    None => rsx! { p { "Loading..." } }
                }
            }
        }
    )
}
