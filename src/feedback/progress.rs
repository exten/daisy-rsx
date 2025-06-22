#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::fmt::Display;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProgressColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for ProgressColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressColor::Default => write!(f, "progress-neutral"),
            ProgressColor::Neutral => write!(f, "progress-neutral"),
            ProgressColor::Primary => write!(f, "progress-primary"),
            ProgressColor::Secondary => write!(f, "progress-secondary"),
            ProgressColor::Accent => write!(f, "progress-accent"),
            ProgressColor::Info => write!(f, "progress-info"),
            ProgressColor::Success => write!(f, "progress-success"),
            ProgressColor::Warning => write!(f, "progress-warning"),
            ProgressColor::Error => write!(f, "progress-error"),
        }
    }
}
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    ui_color: Option<ProgressColor>,
    pub name: String,
    pub id: Option<String>,
    pub value: i16,
    pub max: i16,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let color = props.ui_color.unwrap_or_default();
    let class = format!("{}", color);
    rsx!(progress {
        class: "{class}",
        value: "{props.value}",
        max: "{props.max}"
    })
}

// <div class="radial-progress" style="--value:70; --size:12rem; --thickness: 2px;" aria-valuenow="70" role="progressbar">70%</div>
// <div class="radial-progress" style="--value:70; --size:12rem; --thickness: 2rem;" aria-valuenow="70" role="progressbar">70%</div>
// <div class="radial-progress text-primary" style="--value:70;" aria-valuenow="70" role="progressbar">70%</div>

#[component]
pub fn RadialProgress(props: ProgressProps) -> Element {
    // let alert_color = props.alert_color.unwrap_or_default();
    // let class = props.class.unwrap_or_default();
    // let class = format!("{} {}", alert_color.to_string(), class);

    rsx!(
        div{
            class:"radial-progress bg-primary text-primary-content border-primary border-4",
            style:"--value:{props.value};", aria_valuenow:"{props.value}", role: "progressbar"
            "{props.value}%"
        }
    )
}
