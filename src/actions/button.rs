#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BtnColor {
    #[default]
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for BtnColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BtnColor::Neutral => write!(f, "btn-neutral"),
            BtnColor::Primary => write!(f, "btn-primary"),
            BtnColor::Secondary => write!(f, "btn-secondary"),
            BtnColor::Accent => write!(f, "btn-accent"),
            BtnColor::Info => write!(f, "btn-info"),
            BtnColor::Success => write!(f, "btn-success"),
            BtnColor::Warning => write!(f, "btn-warning"),
            BtnColor::Error => write!(f, "btn-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BtnType {
    Submit,
    Reset,
    Link,
    #[default]
    Button,
}

impl Display for BtnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BtnType::Submit => write!(f, "submit"),
            BtnType::Reset => write!(f, "reset"),
            BtnType::Button => write!(f, "button"),
            BtnType::Link => write!(f, "button"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BtnSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for BtnSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BtnSize::Default => write!(f, "btn-sm"),
            BtnSize::ExtraSmall => write!(f, "btn-xs"),
            BtnSize::Small => write!(f, "btn-sm"),
            BtnSize::Medium => write!(f, "btn-md"),
            BtnSize::Large => write!(f, "btn-lg"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BtnShape {
    #[default]
    None,
    Circle,
    Square,
}

impl Display for BtnShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BtnShape::None => write!(f, ""),
            BtnShape::Circle => write!(f, "btn-circle"),
            BtnShape::Square => write!(f, "btn-square"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BtnStyle {
    #[default]
    None,
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

impl Display for BtnStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BtnStyle::None => write!(f, ""),
            BtnStyle::Outline => write!(f, "btn-outline"),
            BtnStyle::Dash => write!(f, "btn-dash"),
            BtnStyle::Soft => write!(f, "btn-soft"),
            BtnStyle::Ghost => write!(f, "btn-ghost"),
            BtnStyle::Link => write!(f, "btn-link"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BtnProps {
    children: Element,
    id: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    href: Option<String>,
    target: Option<String>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
    btn_type: Option<BtnType>,
    btn_size: Option<BtnSize>,
    btn_color: Option<BtnColor>,
    popover_target: Option<String>,
    popover_target_action: Option<String>,
    disabled_text: Option<String>,
    btn_shape: Option<BtnShape>,
    btn_style: Option<BtnStyle>,
}

#[component]
pub fn Button(props: BtnProps) -> Element {
    let btn_type = props.btn_type.unwrap_or_default();
    let btn_size = props.btn_size.unwrap_or_default();
    let btn_color = props.btn_color.unwrap_or_default();
    let btn_shape = props.btn_shape.unwrap_or_default();
    let btn_style = props.btn_style.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);

    if props.btn_type == Some(BtnType::Link) {
        rsx!(
            a {
                class: "btn {class} {btn_color} {btn_size} {btn_shape} {btn_style}",
                id: props.id,
                href: props.href,
                target: props.target,
                if let Some(img_src) = props.prefix_image_src {
                        img {
                            src: "{img_src}",
                            width: "16"
                        }
                },
                {props.children},
                if let Some(img_src) = props.suffix_image_src {
                        img {
                            src: "{img_src}",
                            width: "16"
                        }
                }
            }
        )
    } else {
        rsx!(
            button {
                class: "btn {class} {btn_color} {btn_size} {btn_shape} {btn_style}",
                id: props.id,
                disabled,
                // We wanted to use popover but doesnt seem to work with daisy modals
                "data-target": props.popover_target,
                "type": "{btn_type}",
                "data-disabled-text": props.disabled_text,
                if let Some(img_src) = props.prefix_image_src {
                    img { src: "{img_src}", width: "16" }
                }
                {props.children}
                if let Some(img_src) = props.suffix_image_src {
                    img { src: "{img_src}", width: "16" }
                }
            }
        )
    }
}

#[test]
fn test_button() {
    let props = BtnProps {
        children: rsx!("Hello"),
        class: Some("test".to_string()),
        href: None,
        target: None,
        btn_color: Some(BtnColor::Primary),
        btn_size: Some(BtnSize::Large),
        btn_type: Some(BtnType::Button),
        btn_shape: None,
        btn_style: None,
        id: Some("id".to_string()),
        disabled: Some(false),
        prefix_image_src: None,
        suffix_image_src: None,
        disabled_text: None,
        popover_target: None,
        popover_target_action: None,
    };

    let expected =
        r#"<button class="btn test btn-primary btn-lg  " id="id" type="button">Hello</button>"#;
    let result = dioxus_ssr::render_element(Button(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

// test button with images
#[test]
fn test_button_with_images() {
    let props = BtnProps {
        children: rsx!("Hello"),
        class: Some("test".to_string()),
        href: None,
        target: None,
        btn_color: Some(BtnColor::Primary),
        btn_size: Some(BtnSize::Large),
        btn_type: Some(BtnType::Button),
        btn_shape: None,
        btn_style: None,
        id: Some("id".to_string()),
        disabled: Some(false),
        prefix_image_src: Some("prefix.png".to_string()),
        suffix_image_src: Some("suffix.png".to_string()),
        disabled_text: None,
        popover_target: None,
        popover_target_action: None,
    };

    let expected = r#"<button class="btn test btn-primary btn-lg  " id="id" type="button"><img src="prefix.png" width="16"/>Hello<img src="suffix.png" width="16"/></button>"#;
    let result = dioxus_ssr::render_element(Button(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

// test all button schemes
#[test]
fn test_all_button_schemes() {
    let schemes = [
        (BtnColor::Neutral, "btn-neutral"),
        (BtnColor::Primary, "btn-primary"),
        (BtnColor::Secondary, "btn-secondary"),
        (BtnColor::Accent, "btn-accent"),
        (BtnColor::Info, "btn-info"),
        (BtnColor::Success, "btn-success"),
        (BtnColor::Warning, "btn-warning"),
        (BtnColor::Error, "btn-error"),
    ];

    for (scheme, expected_class) in schemes {
        let props = BtnProps {
            children: rsx!("Test"),
            class: None,
            href: None,
            target: None,
            btn_color: Some(scheme),
            btn_size: None,
            btn_type: None,
            btn_shape: None,
            btn_style: None,
            id: None,
            disabled: None,
            prefix_image_src: None,
            suffix_image_src: None,
            disabled_text: None,
            popover_target: None,
            popover_target_action: None,
        };

        let result = dioxus_ssr::render_element(Button(props));
        assert!(
            result.contains(expected_class),
            "Expected '{}' to contain '{}', but got: {}",
            result,
            expected_class,
            result
        );
    }
}

// test default button scheme
#[test]
fn test_default_button_scheme() {
    let props = BtnProps {
        children: rsx!("Default"),
        class: None,
        href: None,
        target: None,
        btn_color: None, // Should use default (Neutral)
        btn_size: None,
        btn_type: None,
        btn_shape: None,
        btn_style: None,
        id: None,
        disabled: None,
        prefix_image_src: None,
        suffix_image_src: None,
        disabled_text: None,
        popover_target: None,
        popover_target_action: None,
    };

    let result = dioxus_ssr::render_element(Button(props));
    assert!(
        result.contains("btn-neutral"),
        "Expected default scheme to be 'btn-neutral', but got: {}",
        result
    );
}
