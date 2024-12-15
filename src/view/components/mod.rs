use maud::{html, Markup};

use crate::github::repo::Repo;

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
        button class=(class) hx-post=(path) hx-target="#body" { (text) }
    }
}

pub fn generate_header(columns: Vec<&str>) -> Markup {
    html! {
        table class="table"{
            thead {
                tr {
                    @for column in columns.clone() {
                        th { (column) }
                    }
                }
            }
        }
    }
}

pub fn generate_table(data: Vec<Repo>, columns: Vec<&str>) -> Markup {
    html! {

        table class="table" {
            tbody {
                @for row in data {
                    tr {
                        @for column in columns.clone() {
                            td {
                                @match column {
                                    "Name" => (row.name()),
                                    _ => (""),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
