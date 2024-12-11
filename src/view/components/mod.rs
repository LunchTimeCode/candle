use maud::{html, Markup};

#[allow(dead_code)]
pub fn list_of(any_vec: Vec<Markup>) -> Markup {
    html! {
        div {
                @for m in &any_vec {
                            div { (m) }
                }
        }
    }
}

#[allow(dead_code)]
pub fn grid_of(any_vec: Vec<Markup>) -> Markup {
    html! {
        div .container {
            div class="grid"{
                @for m in &any_vec {
                    div { (m) }
                }
            }
        }
    }
}

pub fn nav_button_with_class(text: &str, path: &str, class: &str) -> Markup {
    html! {
        button class=(class) hx-get=(path) hx-target="#body" { (text) }
    }
}
