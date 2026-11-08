use dioxus::prelude::*;
use dioxus_icons::lucide::{AppWindow, Hash, House, Logs, Signpost};

use crate::{
    backend::nginx_status,
    cards::common::{InfoType, Row},
    components::{
        card::{Card, CardAction, CardContent, CardHeader, CardTitle},
        input::Input,
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
                            ScrollArea {
                                height: "20em",
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
                                        CodePill { code: log.0 }
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

fn code_to_class(code: u16) -> String {
    match code {
        100..200 => "information",
        200..300 => "success",
        300..400 => "redirect",
        400..500 => "client_error",
        500..600 => "server_error",
        _ => "black",
    }
    .to_string()
}

#[component]
pub fn CodePill(code: u16) -> Element {
    let code_class = code_to_class(code);
    rsx! {
        div {
            class: format!("pill {code_class}"),
            {code.to_string()}
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

    let mut search_field = use_signal(String::new);

    use_future(move || async move {
        nginx_logs.set(Some(nginx_status().await.unwrap()));
    });

    rsx!(
        Card {
            big: true,
            tall: true,
            CardHeader {
                CardTitle { Logs {} "Logs" }
                CardAction {
                    Input {
                        oninput: move |e: FormEvent| search_field.set(e.value()),
                        placeholder: "Search",
                        value: search_field,
                        div {"test"}
                    }
                }
            }
            CardContent {
                match &*nginx_logs.read_unchecked() {
                    Some(logs) => {
                        rsx! {
                            ScrollArea {
                                height: "20em",
                                for log in logs.logs.iter().filter(|log| log.to_string().contains(&*search_field.read())) {
                                    div {
                                        class: "nginx-log-line",
                                        span { "{log.timestamp}"} CodePill { code: log.status_code } span {"{log.http_method} {log.host} {log.http_path} {log.user_agent}" }
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
