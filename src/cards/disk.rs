use dioxus::prelude::*;

use crate::{
    backend::{disks_info, Disks},
    cards::common::{percent, CardTitle, ProgressBar},
};

#[component]
pub fn DisksCard() -> Element {
    let mut disks = use_signal(Disks::default);

    use_future(move || async move {
        disks.set(disks_info().await.unwrap());
    });

    rsx! {
        div {
            class: "card",
            CardTitle { title: "💽 Disks" },
            for disk in disks.read().iter().filter(|d| !d.mount_point.starts_with("/var")) {
                {
                    let available_go = disk.available_space as f64 / 10u64.pow(9) as f64;
                    let total_go = disk.total_space as f64 / 10u64.pow(9) as f64;
                    let used_go = total_go - available_go;
                    let percent = percent(disk.total_space - disk.available_space, disk.total_space);
                    rsx! {
                        div {
                            class: "disk",
                            div {
                                class: "disk-main-info",
                                span { class: "mount-point accent", "{disk.mount_point}" }
                                span { class: "disk-type", "{disk.kind}" }
                            }
                            ProgressBar { progression: disk.total_space - disk.available_space , total: disk.total_space }
                            div {
                                class: "disk-main-info",
                                span { class: "note", "{used_go:.2} / {total_go:.2} Go ({percent:.2}%" }
                                span { class: "note", "{disk.file_system}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
