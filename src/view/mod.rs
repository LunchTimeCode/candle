use maud::{html, Markup, PreEscaped};
use rocket::response::content;

mod components;
pub mod dashboard;
pub mod nav;

#[get("/")]
pub fn index() -> content::RawHtml<String> {
    let raw = page(html! {
        header {
            // load nav always
            div hx-get="/nav" hx-trigger="load" {}
            // load home first
            div hx-get="/dashboard" hx-trigger="load" hx-target="#body" {}
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
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}
