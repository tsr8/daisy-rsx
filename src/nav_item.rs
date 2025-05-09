#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavItemProps {
    href: String,
    icon: String,
    title: String,
    selected_item_id: Option<String>,
    id: Option<String>,
}

#[component]
pub fn NavItem(props: NavItemProps) -> Element {
    let class = match (&props.id, &props.selected_item_id) {
        (Some(id), Some(selected_id)) if id == selected_id => "active",
        _ => "",
    };
    rsx!(
        li { role: "listitem",
            a {
                class: "{class}",
                href: "{props.href}",
                "data-turbo-frame": "main-content",
                img { width: "16", height: "16", src: "{props.icon}" }
                "{props.title}"
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct NavSubItemProps {
    href: String,
    title: String,
    selected_item_id: Option<String>,
    id: Option<String>,
}

#[component]
pub fn NavSubItem(props: NavSubItemProps) -> Element {
    let class = match (&props.id, &props.selected_item_id) {
        (Some(id), Some(selected_id)) if id == selected_id => "active",
        _ => "",
    };

    rsx!(
        li { class,
            a { href: "{props.href}", "{props.title}" }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct NavGroupProps {
    heading: String,
    content: Element,
}

#[component]
pub fn NavGroup(props: NavGroupProps) -> Element {
    rsx!(
        ul { role: "list", class: "menu",
            li { class: "menu-title", "{props.heading}" }
            {props.content}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct NavSubGroupProps {
    children: Element,
}

#[component]
pub fn NavSubGroup(props: NavSubGroupProps) -> Element {
    rsx!(
        ul { role: "list", class: "ActionList ActionList--subGroup", {props.children} }
    )
}

#[test]
fn test_nav_item() {
    let props = NavItemProps {
        href: "test".to_string(),
        icon: "test".to_string(),
        title: "test".to_string(),
        selected_item_id: Some("test".to_string()),
        id: Some("test".to_string()),
    };

    let expected = r#"<li role="listitem"><a class="active" href="test" data-turbo-frame="main-content"><img width="16" height="16" src="test"/>test</a></li>"#;
    let result = dioxus_ssr::render_element(NavItem(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
