use dioxus::prelude::*;

use crate::{
    backend::docker_info,
    cards::{
        disk::DisksCard, docker::DockerCard, memory::MemoryCard, system::SystemCard,
        temperature::TemperatureCard,
    },
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const LOGO: Asset = asset!("/assets/dashberry.svg");

mod backend;
mod cards;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut docker_available = use_signal(|| false);

    use_future(move || async move {
        docker_available.set(docker_info().await.is_ok());
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            id: "header",
            img { id: "header-icon", src: LOGO }
            span { "Dashberry" }
        }

        div {
            id: "main-grid",
            SystemCard { }
            TemperatureCard {  }
            MemoryCard {  }
            DisksCard {  }
            {
                if *docker_available.read() {
                    rsx! { DockerCard {  } }
                } else {
                    rsx! { }
                }
            }
        }

    }
}
