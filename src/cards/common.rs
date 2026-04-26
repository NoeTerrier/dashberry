use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfoType {
    Default,
    Note,
    Highlight,
    Accent,
}

impl InfoType {
    fn to_css_class(&self) -> &str {
        match self {
            InfoType::Default => "default",
            InfoType::Note => "note",
            InfoType::Highlight => "highlight",
            InfoType::Accent => "accent",
        }
    }
}

#[component]
pub fn Row(label: String, value: String, info_type: InfoType) -> Element {
    rsx!(
        div {
            class: "row",
            span { class: "label", "{label}" },
            span { class: info_type.to_css_class(), "{value}" }
        }
    )
}

#[component]
pub fn CardTitle(title: String) -> Element {
    rsx!(
        div {
            class: "card-title",
            "{title}"
        }
    )
}

#[component]
pub fn ProgressBar(progression: u64, total: u64) -> Element {
    let p = percent(progression, total) as u64;
    rsx!(
        div {
            class: "progressbar",
            div {
                class: "progression",
                width: "{p}%"
            }
        }
    )
}

pub fn percent(used: u64, total: u64) -> f64 {
    (used as f64) / (total as f64) * 100f64
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagType {
    Ok,
    Warning,
    Error,
    Disabled,
}

impl TagType {
    fn to_css_class(&self) -> &str {
        match self {
            TagType::Ok => "ok",
            TagType::Warning => "warning",
            TagType::Error => "error",
            TagType::Disabled => "disabled",
        }
    }
}

#[component]
pub fn Tag(text: String, tag_type: TagType, short_text: String) -> Element {
    rsx! {
        div {
            class: "tag {tag_type.to_css_class()}",
            Dot { tag_type: tag_type.clone() }
            span { class: "tag-text", "{text}" }
            span { class: "tag-short-text", "{short_text.to_uppercase()}" }
        }
    }
}

#[component]
pub fn Dot(tag_type: TagType) -> Element {
    rsx! {
        span { class: "status-dot {tag_type.to_css_class()}" }
    }
}
