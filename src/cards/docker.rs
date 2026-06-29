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
            class: "card big",
            CardTitle { title: "🐳 Docker containers" }
            div { class: "docker-list",
                for container in &*containers.read() {
                    {
                        let name = container.name.first().and_then(|s| s.strip_prefix('/')).unwrap_or("No name");
                        rsx! {
                            div { class: "container",
                                img { style: "fill: white;", src: get_icon(&container.project_name) }
                                div { class: "container-info",
                                    div { class: "row",
                                        span { "{name}" }
                                        Tag {
                                            text: "{container.status}",
                                            short_text: "{container.state}",
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

fn get_icon(image_name: &Option<String>) -> String {    
    if let Some(image_name) = image_name {
        format!("https://cdn.jsdelivr.net/gh/selfhst/icons@main/svg/{image_name}.svg")
    } else {
        asset!("assets/container.svg").to_string()
    }
}
