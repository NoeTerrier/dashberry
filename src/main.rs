use crate::backend::nginx_status;
use crate::cards::nginx::{
    NginxCodeCard, NginxHostsCard, NginxLogsCard, NginxPathCard, NginxUserAgentsCard,
};
use crate::components::tabs::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Computer, Container, Hexagon};

use crate::{
    backend::docker_info,
    cards::{
        disk::DisksCard, docker::DockerCard, memory::MemoryCard, network::NetworkCard,
        system::SystemCard, temperature::TemperatureCard,
    },
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.scss");
const LOGO: Asset = asset!("/assets/dashberry.svg");
#[used]
static FONT: Asset = asset!(
    "/assets/ProFont.ttf",
    AssetOptions::builder().with_hash_suffix(false)
);

mod backend;
mod cards;
mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut docker_available = use_signal(|| false);
    let mut nginx_available = use_signal(|| false);

    use_future(move || async move {
        docker_available.set(docker_info().await.is_ok());
    });

    use_future(move || async move {
        nginx_available.set(nginx_status().await.is_ok());
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            id: "header",
            img { id: "header-icon", src: LOGO }
            span { "Dashberry" }
        }

        Tabs {
            default_value: "system",
            variant: TabsVariant::Default,
            TabList {
                TabTrigger { value: "system", index: 0_usize, Computer { style: "height: 1em; width: 1em;" } "System" }

                {
                    if *docker_available.read() {
                        rsx! { TabTrigger { value: "nginx", index: 1_usize, Hexagon { style: "height: 1em; width: 1em;" } "Nginx" }}
                    } else {
                        rsx! { }
                    }
                }
                {
                    if *docker_available.read() {
                        rsx! { TabTrigger { value: "docker", index: 2_usize, Container { style: "height: 1em; width: 1em;" } "Docker" } }
                    } else {
                        rsx! { }
                    }
                }
            }
            TabContent {
                index: 0_usize,
                value: "system",
                div {
                    id: "main-grid",
                    SystemCard { }
                    TemperatureCard {  }
                    MemoryCard {  }
                    NetworkCard {  }
                    DisksCard {  }
                }
            }
            TabContent {
                index: 1_usize,
                value: "nginx",
                div {
                    id: "main-grid",
                    NginxPathCard {}
                    NginxCodeCard {}
                    NginxLogsCard {}
                    NginxUserAgentsCard {}
                    NginxHostsCard {}
                }
            }
            TabContent {
                index: 2_usize,
                value: "docker",
                div {
                    id: "main-grid",
                    DockerCard {}
                }
            }
        }
    }
}
