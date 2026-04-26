use dioxus::prelude::*;

use crate::{
    backend::network_info,
    cards::common::{CardTitle, InfoType, Row, Tag, TagType},
};

#[component]
pub fn NetworkCard() -> Element {
    let mut interfaces = use_signal(|| None);

    use_future(move || async move {
        let mut i = network_info().await.unwrap();
        i.sort_by_key(|i| i.name.clone());

        interfaces.set(Some(i));
    });

    rsx! {
        div {
            class: "card",
            CardTitle { title: "🛜 Network interfaces" },
            div { class: "network-list",
                match &*interfaces.read() {
                    Some(interfaces) =>
                        rsx! {
                            for interface in &interfaces {{
                                rsx! {
                                    div {
                                        class: "interface",
                                        div {
                                            class: "row",
                                            span { class: "accent", "{interface.name}" }
                                            if interface.is_up {
                                                Tag { text: "UP", tag_type: TagType::Ok, short_text: "UP" }
                                            } else {
                                                Tag { text: "DOWN", tag_type: TagType::Disabled, short_text: "DOWN" }
                                            }
                                        }
                                        if let Some(mac) = &interface.mac_addr {
                                            Row { label: "MAC", value: "{mac}", info_type: InfoType::Note }
                                        }
                                        for ip in &interface.ipv4 {
                                            Row { label: "ipv4", value: "{ip}", info_type: InfoType::Default }
                                        }
                                        for ip in &interface.ipv6 {
                                            Row { label: "ipv6", value: "{ip}", info_type: InfoType::Default }
                                        }
                                    }
                                }
                            }}
                        },
                    _ => rsx! { p { "Loading..." } }
                }
            }
        }
    }
}
