#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::BtnColor;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum DialogType {
    #[default]
    Default,
    Checkbox,
    AnchorLink,
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    #[props(default = String::new())]
    trigger_id: String,

    #[props(default = String::new())]
    submit_action: String,

    #[props(default = String::new())]
    class: String,

    #[props(default = BtnColor::Info)]
    btn_color: BtnColor,

    #[props(default = DialogType::Checkbox)]
    modal_type: DialogType,

    children: Element,
}

/// There are 3 methods to use modals
/// 1. Using HTML <dialog> element
///    It needs JS to open but it has better accessibility and we can close it usingEsckey
/// 2. Using checkbox
///    A hidden<input type="checkbox">to control the state of modal and<label>to check/uncheck the checkbox and open/close the modal
/// 3. Using <a> anchor links
///    A link adds a parameter to the URL and you only see the modal when the URL has that parameter
///
#[component]
pub fn Modal(props: ModalProps) -> Element {
    match props.modal_type {
        // <button class="btn" onclick="my_modal_2.showModal()">open modal</button>
        DialogType::Default => Default(props),
        // <a href="#my_modal_8" class="btn">open modal</a>
        DialogType::AnchorLink => AnchorLink(props),
        // <label for="my_modal_7" class="btn">open modal</label>
        DialogType::Checkbox => Checkbox(props),
    }
}

fn Default(props: ModalProps) -> Element {
    rsx!(
        dialog { id:"my_modal_2", class:"modal",
            div {class:"modal-box",
                h3 { class:"text-lg font-bold", "Hello!"}
                p { class:"py-4", "Press ESC key or click outside to close"}
            }
            form { method:"dialog", class:"modal-backdrop",
                    button {"close"}
            }
        },
    )
}

fn Checkbox(props: ModalProps) -> Element {
    rsx!(
        label { r#for: "my_modal_7", class: "btn", "Open" }
        input { r#type: "checkbox", class: "modal-toggle", id: "my_modal_7" }
        div { role: "dialog", class: "modal",
            div {
                class: "modal-box",
                h3 { class: "text-lg font-bold", "Title" }
                p { class: "py-4", "content ....." }
            }
            label { r#for: "my_modal_7", class: "modal-backdrop", "Close" }
        }
    )
}

fn AnchorLink(props: ModalProps) -> Element {
    rsx!(
        a { href: "#my_modal_8", class: "btn", "Open" }
        div { role: "dialog", class: "modal", id: "my_modal_8",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Title" }
                p { class: "py-4", "This modal works with anchor links" }
                div { class: "modal-action",
                    a { href: "#", class: "btn", "Yal!" }
                }
            }
        }
    )
}

#[component]
pub fn Modal_1(props: ModalProps) -> Element {
    rsx!()
    // match props.submit_action {
    //     Some(action) => {
    //         rsx!(
    //             form { action: "{action}", method: "post",
    //                 dialog {
    //                     class: "modal {props.class.clone().unwrap_or_default()}",
    //                     id: "{props.trigger_id}",
    //                     popover: "auto",
    //                     {props.children}
    //                 }
    //             }
    //         )
    //     }
    //     _ => {
    //         rsx!(
    //             dialog {
    //                 class: "modal {props.class.clone().unwrap_or_default()}",
    //                 id: "{props.trigger_id}",
    //                 popover: "auto",
    //                 {props.children}
    //             }
    //         )
    //     }
    // }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn ModalBody(props: ModalBodyProps) -> Element {
    rsx!(
        div { class: "modal-box {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalActionProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn ModalAction(props: ModalActionProps) -> Element {
    rsx!(
        div { class: "modal-action {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}

#[test]
fn test_modal() {
    let props = ModalProps {
        children: rsx!("Hello"),
        class: Some("test".to_string()),
        submit_action: Some("test".to_string()),
        trigger_id: "id".to_string(),
    };

    let expected = r#"<form action="test" method="post"><dialog class="modal test" id="id" popover="auto">Hello</dialog></form>"#;
    let result = dioxus_ssr::render_element(Modal(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

#[test]
fn test_modal_without_submit_action() {
    let props = ModalProps {
        children: rsx!("Hello"),
        class: Some("test".to_string()),
        submit_action: None,
        trigger_id: "id".to_string(),
    };

    let expected = r#"<dialog class="modal test" id="id" popover="auto">Hello</dialog>"#;
    let result = dioxus_ssr::render_element(Modal(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
