use maud::{html, Markup, PreEscaped};
use rocket::response::content;

mod components;
pub mod dashboard;
pub mod nav;
pub mod org_changed;

#[derive(FromForm)]
pub struct OrgForm {
    org: String,
}

#[get("/")]
pub async fn index() -> content::RawHtml<String> {
    let raw = page(html! {
        header {
            // load nav always
            div hx-get="/nav" hx-trigger="load" {}
            // load home first
            div hx-post="/dashboard" hx-trigger="load" hx-target="#body" {}
        }

    body {
        div #body {}
    }
    })
    .into_string();
    content::RawHtml(raw)
}

const CSS: &str = r#"<link rel="stylesheet" href="_assets/app.css">"#;
const PICO_EXT: &str = r#"<link rel="stylesheet" href="_assets/pico_ext.css">"#;
const HTMX: &str = r#"<script src="/_assets/htmx.min.js"></script>"#;
const PICO: &str = r#"<link rel="stylesheet" href="_assets/pico.violet.min.css">"#;
const ICON: &str =
    r#"<link rel="icon" type="image/x-icon" href="https://i.ibb.co/RDThxRB/favicon-32x32.png">"#;

pub fn page(markup: Markup) -> Markup {
    html! {
       html {

            head {
                ({scripts()})
                ({title("Candle")})

            }

            body class="container" {
                (markup)
            }
        }
    }
}

fn scripts() -> Markup {
    html! {
       (PreEscaped(CSS))
       (PreEscaped(HTMX))
       (PreEscaped(PICO_EXT))
       (PreEscaped(PICO))
       (PreEscaped(ICON))
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}
