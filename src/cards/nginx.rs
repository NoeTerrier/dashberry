use dioxus::prelude::*;
use dioxus_icons::lucide::{AppWindow, Hash, House, Logs, Signpost};

use crate::{
    backend::nginx_status,
    cards::common::{InfoType, Row, Tag, TagType},
    components::{
        card::{Card, CardContent, CardHeader, CardTitle},
        scroll_area::ScrollArea,
    },
};

#[component]
pub fn NginxPathCard() -> Element {
    let mut nginx_logs = use_signal(|| None);

    use_future(move || async move {
        nginx_logs.set(Some(nginx_status().await.unwrap()));
    });

    rsx!(
        Card {
            CardHeader {
                CardTitle { Signpost {} "Paths" }
            }
            CardContent {
                match &*nginx_logs.read_unchecked() {
                    Some(logs) => {
                        rsx! {
                            div {
                                for log in &logs.http_paths {
                                    Row { label: &log.0, value: &log.1, info_type: InfoType::Accent }
                                }
                            }
                        }
                    },
                    None => rsx! { p { "Loading..." } }
                }
            }
        }
    )
}

#[component]
pub fn NginxCodeCard() -> Element {
    let mut nginx_logs = use_signal(|| None);

    use_future(move || async move {
        nginx_logs.set(Some(nginx_status().await.unwrap()));
    });

    rsx!(
        Card {
            CardHeader {
                CardTitle { Hash {} "Status Code" }
            }
            CardContent {
                match &*nginx_logs.read_unchecked() {
                    Some(logs) => {
                        rsx! {
                            div {
                                for log in &logs.status_codes {
                                    div {
                                        class: "row",
                                        Pill { color: "green".to_string(), text: log.0.to_string() }
                                        span { "{log.1}" }
                                    }
                                }
                            }
                        }
                    },
                    None => rsx! { p { "Loading..." } }
                }
            }
        }
    )
}

#[component]
pub fn Pill(color: String, text: String) -> Element {
    rsx! {
        div {
            class: "pill",
            style: "background-color: {color}",
            {text}
        }
    }
}

#[component]
pub fn NginxUserAgentsCard() -> Element {
    let mut nginx_logs = use_signal(|| None);

    use_future(move || async move {
        nginx_logs.set(Some(nginx_status().await.unwrap()));
    });

    rsx!(
        Card {
            big: true,
            CardHeader {
                CardTitle { AppWindow {} "User Agents" }
            }
            CardContent {
                match &*nginx_logs.read_unchecked() {
                    Some(logs) => {
                        rsx! {
                            div {
                                for log in &logs.user_agents {
                                    Row { label: &log.0, value: &log.1, info_type: InfoType::Accent }
                                }
                            }
                        }
                    },
                    None => rsx! { p { "Loading..." } }
                }
            }
        }
    )
}

#[component]
pub fn NginxHostsCard() -> Element {
    let mut nginx_logs = use_signal(|| None);

    use_future(move || async move {
        nginx_logs.set(Some(nginx_status().await.unwrap()));
    });

    rsx!(
        Card {
            CardHeader {
                CardTitle { House {} "Hosts" }
            }
            CardContent {
                match &*nginx_logs.read_unchecked() {
                    Some(logs) => {
                        rsx! {
                            div {
                                for log in &logs.hosts {
                                    Row { label: &log.0, value: &log.1, info_type: InfoType::Accent }
                                }
                            }
                        }
                    },
                    None => rsx! { p { "Loading..." } }
                }
            }
        }
    )
}

#[component]
pub fn NginxLogsCard() -> Element {
    let mut nginx_logs = use_signal(|| None);

    use_future(move || async move {
        nginx_logs.set(Some(nginx_status().await.unwrap()));
    });

    rsx!(
        Card {
            big: true,
            CardHeader {
                CardTitle { Logs {} "Logs" }
            }
            CardContent {
                match &*nginx_logs.read_unchecked() {
                    Some(logs) => {
                        rsx! {
                            ScrollArea {
                                height: "10em",
                                for log in &logs.logs {
                                    p { "{log.timestamp} {log.status_code} {log.http_method} {log.host} {log.http_path} {log.user_agent}" }
                                }
                            }
                        }
                    },
                    None => rsx! { p { "Loading..." } }
                }
            }
        }
    )
}
