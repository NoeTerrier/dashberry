use dioxus::prelude::*;

use crate::{
    backend::{docker_info, Containers},
    cards::common::{CardTitle, Tag, TagType},
};

const ICONS_FOLDER: Asset = asset!("/assets/icons");

#[component]
pub fn DockerCard() -> Element {
    let mut containers = use_signal(Containers::default);

    use_future(move || async move {
        containers.set(docker_info().await.unwrap());
    });

    rsx! {
        div {
            class: "card big",
            CardTitle { title: "🐳 Docker containers" }
            div { class: "docker-list",
                for container in &*containers.read() {
                    {
                        let name = container.name.first().and_then(|s| s.strip_prefix('/')).unwrap_or("No name");
                        rsx! {
                            div { class: "container",
                                img { style: "fill: white;", src: get_icon(&container.image) }
                                div { class: "container-info",
                                    div { class: "row",
                                        span { class: "container-name", "{name}" }
                                        Tag {
                                            text: "{container.status}",
                                            short_text: "{container.state}",
                                            color: "red",
                                            tag_type: container_state_tag_type(&container.state),
                                        }
                                    }
                                    span { class: "container-name note", "{container.image}" }
                                }
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

fn get_icon(image_name: &str) -> String {
    let mut image_name = image_name.to_string();

    if image_name.contains("\\") {
        image_name = image_name
            .chars()
            .skip_while(|c| *c != '/')
            .collect::<String>();
    }

    if image_name.contains(":") {
        image_name = image_name
            .chars()
            .take_while(|c| *c != ':')
            .collect::<String>();
    }

    let known_app_names = ["nginx", "immich", "tailscale", "postgres"];

    for name in known_app_names {
        if image_name.contains(name) {
            return format!("{ICONS_FOLDER}/{name}.svg");
        }
    }

    asset!("assets/container.svg").to_string()
}
