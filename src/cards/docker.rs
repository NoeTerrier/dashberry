use dioxus::prelude::*;

use crate::{
    backend::{docker_info, Containers},
    cards::common::{CardTitle, Tag, TagType},
};

#[component]
pub fn DockerCard() -> Element {
    let mut containers = use_signal(Containers::default);

    use_future(move || async move {
        containers.set(docker_info().await.unwrap());
    });

    rsx! {
        div {
            class: "card",
            CardTitle { title: "🐳 Docker containers" }
            for container in &*containers.read() {
                div { class: "container",
                    img { style: "fill: white;", src: get_icon(&container.image) }
                    div { class: "container-info",
                        span { class: "image-name", "{container.image}" }
                        Tag {
                            text: "{container.state}",
                            color: "red",
                            tag_type: container_state_tag_type(&container.state)
                        }
                    }
                }
            }
            if containers.read().is_empty() {
                p { class: "note", "[ NO CONTAINER ]" }
            }
        }
    }
}

fn container_state_tag_type(state: &str) -> TagType {
    match state {
        "exited" => TagType::Disabled,
        "running" | "created" => TagType::Ok,
        "paused" => TagType::Warning,
        "dead" => TagType::Error,
        _ => TagType::Error,
    }
}

fn get_icon(app_name: &str) -> Asset {
    if app_name.contains("ngninx") {
        asset!("/assets/icons/ngninx.svg")
    } else if app_name.contains("immich") {
        asset!("/assets/icons/immich.svg")
    } else if app_name.contains("tailscale") {
        asset!("/assets/icons/tailscale.svg")
    } else if app_name.contains("postgres") {
        asset!("/assets/icons/postgresql.svg")
    } else {
        asset!("/assets/container.svg")
    }
}
