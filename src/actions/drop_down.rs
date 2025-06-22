#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    #[default]
    None,
    Start,
    Center,
    End,
    Top,
    Bottom,
    Left,
    Right,
}
// Positions
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::None => write!(f, "dropdown-bottom"),
            Direction::Start => write!(f, "dropdown-start"),
            Direction::Center => write!(f, "dropdown-center"),
            Direction::End => write!(f, "dropdown-end"),
            Direction::Top => write!(f, "dropdown-top"),
            Direction::Bottom => write!(f, "dropdown-bottom"),
            Direction::Left => write!(f, "dropdown-left"),
            Direction::Right => write!(f, "dropdown-right"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownProps {
    child: Element,
    haver: Option<bool>,
    carat: Option<bool>,
    btn_text: String,
    class: Option<String>,
    direction: Option<Direction>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
}

#[component]
pub fn DropDown(props: DropDownProps) -> Element {
    let direction = props.direction.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let haver = if Some(true) == props.haver {
        "dropdown-hover"
    } else {
        ""
    };
    let class = format!(" {} {} ", class, haver);
    rsx!(
        div { class: "dropdown {class} {direction}",
            div {tabindex: "0", role:"button", class:"btn m-1", "Click" }
            ul {
                tabindex: "0",
                class: "dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm",
                li { a { "Item 1" } }
                li { a { "Item 2" } }
                li { a { "Item 3" } }
                // {props.children}
            }
        }
    )
}

// // 1. Dropdown using details and summary
// <details class="dropdown">
//   <summary class="btn m-1">open or close</summary>
//   <ul class="menu dropdown-content bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
//     <li><a>Item 1</a></li>
//     <li><a>Item 2</a></li>
//   </ul>
// </details>
//
//
// // 1.popover API and anchor
// <!-- change popover-1 and --anchor-1 names. Use unique names for each dropdown -->
// <button class="btn" popovertarget="popover-1" style="anchor-name:--anchor-1">
//   Button
// </button>
// <ul class="dropdown menu w-52 rounded-box bg-base-100 shadow-sm"
//   popover id="popover-1" style="position-anchor:--anchor-1">
//   <li><a>Item 1</a></li>
//   <li><a>Item 2</a></li>
// </ul>
//
//
// // 1.
// <!-- change popover-1 and --anchor-1 names. Use unique names for each dropdown -->
// <button class="btn" popovertarget="popover-1" style="anchor-name:--anchor-1">
//   Button
// </button>
// <ul class="dropdown menu w-52 rounded-box bg-base-100 shadow-sm"
//   popover id="popover-1" style="position-anchor:--anchor-1">
//   <li><a>Item 1</a></li>
//   <li><a>Item 2</a></li>
// </ul>
//
//
// // 1. Force open
// <div class="dropdown dropdown-open">
//   <div tabindex="0" role="button" class="btn m-1">Button</div>
//   <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
//     <li><a>Item 1</a></li>
//     <li><a>Item 2</a></li>
//   </ul>
// </div>
//
//
// // 1. Card as dropdown
// <div class="dropdown">
//   <div tabindex="0" role="button" class="btn m-1">Click</div>
//   <div
//     tabindex="0"
//     class="dropdown-content card card-sm bg-base-100 z-1 w-64 shadow-md">
//     <div class="card-body">
//       <p>This is a card. You can use any element as a dropdown.</p>
//     </div>
//   </div>
// </div>
//
//
// // 2. Helper dropdown
// A normal text and a helper dropdown
// <div class="dropdown dropdown-end">
//   <div tabindex="0" role="button" class="btn btn-circle btn-ghost btn-xs text-info">
//     <svg
//       tabindex="0"
//       xmlns="http://www.w3.org/2000/svg"
//       fill="none"
//       viewBox="0 0 24 24"
//       class="h-4 w-4 stroke-current">
//       <path
//         stroke-linecap="round"
//         stroke-linejoin="round"
//         stroke-width="2"
//         d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
//     </svg>
//   </div>
//   <div
//     tabindex="0"
//     class="card card-sm dropdown-content bg-base-100 rounded-box z-1 w-64 shadow-sm">
//     <div tabindex="0" class="card-body">
//       <h2 class="card-title">You needed more info?</h2>
//       <p>Here is a description!</p>
//     </div>
//   </div>
// </div>

#[component]
pub fn DropDown1(props: DropDownProps) -> Element {
    let direction = props.direction.unwrap_or_default();

    rsx!(
        div { class: "dropdown {props.class.clone().unwrap_or_default()} {direction}",
            label {
                tabindex: "0",
                class: "btn btn-default btn-sm m-1 w-full flex flex-nowrap justify-between",
                "aria-haspopup": "true",
                if let Some(img_src) = props.prefix_image_src {
                    img { src: "{img_src}", class: "mr-2", width: "16" }
                }
                span { class: "truncate", "{props.button_text}" }
                if let Some(img_src) = props.suffix_image_src {
                    img { src: "{img_src}", class: "ml-2", width: "12" }
                } else if props.carat.is_some() && props.carat.unwrap() {
                    div { class: "dropdown-caret" }
                }
            }
            ul {
                tabindex: "0",
                class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 {direction}",
                {props.child}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownLinkProps {
    href: String,
    target: Option<String>,
    popover_target: Option<String>,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn DropDownLink(props: DropDownLinkProps) -> Element {
    let class = format!("dropdown-item {}", props.class.unwrap_or_default());

    if let Some(trigger) = &props.popover_target {
        rsx!(
            li {
                a {
                    class: "{class}",
                    "data-target": "{trigger}",
                    target: props.target,
                    href: "{props.href}",
                    {props.children}
                }
            }
        )
    } else {
        rsx!(
            li {
                a {
                    class: "{class}",
                    target: props.target,
                    href: "{props.href}",
                    {props.children}
                }
            }
        )
    }
}

#[test]
fn test_modal() {
    let props = DropDownProps {
        child: rsx!("Hello"),
        haver: Some(true),
        carat: Some(true),
        btn_text: "".to_string(),
        class: Some(String::new()),
        direction: Some(Direction::Start),
        prefix_image_src: Some(String::new()),
        suffix_image_src: Some(String::new()),
    };

    let expected = r#"<form action="test" method="post"><dialog class="modal test" id="id" popover="auto">Hello</dialog></form>"#;
    let result = dioxus_ssr::render_element(DropDown(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
