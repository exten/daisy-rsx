#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::fmt::Display;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadingSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for LoadingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingSize::Default => write!(f, "Loading-sm"),
            LoadingSize::ExtraSmall => write!(f, "loading-xs"),
            LoadingSize::Small => write!(f, "loading-sm"),
            LoadingSize::Large => write!(f, "loading-lg"),
            LoadingSize::Medium => write!(f, "loading-md"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadingStyle {
    #[default]
    Default,
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}

impl Display for LoadingStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingStyle::Default => write!(f, "loading-spinner"),
            LoadingStyle::Spinner => write!(f, "loading-spinner"),
            LoadingStyle::Dots => write!(f, "loading-dots"),
            LoadingStyle::Ring => write!(f, "loading-ring"),
            LoadingStyle::Ball => write!(f, "loading-ball"),
            LoadingStyle::Bars => write!(f, "loading-bars"),
            LoadingStyle::Infinity => write!(f, "loading-infinity"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadingColor {
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

impl Display for LoadingColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingColor::Default => write!(f, "text-neutral"),
            LoadingColor::Neutral => write!(f, "text-neutral"),
            LoadingColor::Primary => write!(f, "text-primary"),
            LoadingColor::Secondary => write!(f, "text-secondary"),
            LoadingColor::Accent => write!(f, "text-accent"),
            LoadingColor::Info => write!(f, "text-info"),
            LoadingColor::Success => write!(f, "text-success"),
            LoadingColor::Warning => write!(f, "text-waring"),
            LoadingColor::Error => write!(f, "text-error"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LoadingProps {
    ui_size: Option<LoadingSize>,
    ui_style: Option<LoadingStyle>,
    ui_color: Option<LoadingColor>,
    pub name: String,
    pub id: Option<String>,
    pub label_class: Option<String>,
    pub value: Option<String>,
}

#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let sizes = props.ui_size.unwrap_or_default();
    let style = props.ui_style.unwrap_or_default();
    let color = props.ui_color.unwrap_or_default();

    let class = format!("{} {} {}", sizes, style, color);

    rsx!(span { class: "{class}" })
}
