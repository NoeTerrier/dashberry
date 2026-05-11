use dioxus::prelude::*;

use crate::{
    backend::{network_info, Interface, Ipv4, Ipv6},
    cards::common::{CardTitle, Filter, InfoType, Row, Tag, TagType},
};

#[component]
pub fn NetworkCard() -> Element {
    let mut interfaces_signal = use_signal(|| None);

    use_future(move || async move {
        let mut i = network_info().await.unwrap();
        i.sort_by_key(|i| i.name.clone());

        interfaces_signal.set(Some(i));
    });

    let selected_signal = use_signal(|| "Public".to_string());

    let selected = &*selected_signal.read();

    let kind = Kind::from(selected);

    // Filter out interfaces with no address that match filter
    let interfaces: Option<Vec<Interface>> = interfaces_signal.read().clone().map(|v| {
        v.into_iter()
            .filter(|i| {
                i.ipv4.iter().any(|ip| is_kind_ip_v4(&ip, kind))
                    || i.ipv6.iter().any(|ip| is_kind_ip_v6(&ip, kind))
            })
            .collect()
    });

    rsx! {
        div {
            class: "card big",
            div {
                class: "card-header",
                CardTitle { title: "🛜 Network interfaces" },
                Filter {
                    options: vec!["All".to_string(), "Public".to_string(), "Private".to_string(), "IPv4".to_string(), "IPv6".to_string()],
                    selected: selected_signal
                },
            },
            div { class: "network-list",
                match interfaces {
                    Some(interfaces) =>
                        rsx! {
                            for interface in interfaces {{
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
                                        for ip in interface.ipv4.iter().filter(|ip| is_kind_ip_v4(ip, kind)) {
                                            Row { label: "ipv4", value: "{ip.addr}", info_type: InfoType::Default }
                                        }
                                        for ip in interface.ipv6.iter().filter(|ip| is_kind_ip_v6(ip, kind)) {
                                            Row { label: "ipv6", value: "{ip.addr}", info_type: InfoType::Default }
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

#[derive(Clone, Copy)]
enum Kind {
    All,
    Public,
    Private,
    IPv4,
    IPv6,
}

impl Kind {
    fn from(v: &str) -> Self {
        match v {
            "Private" => Kind::Private,
            "Public" => Kind::Public,
            "IPv4" => Kind::IPv4,
            "IPv6" => Kind::IPv6,
            _ => Kind::All,
        }
    }
}

fn is_kind_ip_v4(ip: &&Ipv4, kind: Kind) -> bool {
    match kind {
        Kind::All => true,
        Kind::Public => !ip.is_loopback && !ip.is_private,
        Kind::Private => !ip.is_loopback && ip.is_private,
        Kind::IPv4 => true,
        Kind::IPv6 => false,
    }
}

fn is_kind_ip_v6(ip: &&Ipv6, kind: Kind) -> bool {
    match kind {
        Kind::All => true,
        Kind::Public => !ip.is_loopback && !ip.is_unicast_link_local,
        Kind::Private => !ip.is_loopback && ip.is_unicast_link_local,
        Kind::IPv6 => true,
        Kind::IPv4 => false,
    }
}
