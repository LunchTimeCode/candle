use maud::{html, Markup};
use rocket::{response::content, State};

use crate::{
    github::organization::Organization, state::AppState, view::components::nav_button_with_class,
};

#[get("/nav")]
pub async fn get(state: &State<AppState>) -> content::RawHtml<String> {
    let orgs = state.fetch_organizations().await;
    let current_name = match state.current_org().await {
        Some(o) => o.name(),
        None => {
            let co = orgs.clone();
            co.first()
                .map(|f| f.name())
                .unwrap_or("No organization found".to_string())
        }
    };

    let raw = html! {
        nav {
            ul {
                li {
            a href="/" {
                    div .grid{
                        h1 .nowrap { "Candle" }

                            img src="https://i.ibb.co/8n3TxFT/small-candle.jpg"
                            .left-margin
                            alt="1959be0f-b7f2-4a2d-b719-f5243ad84152"
                            border="0"
                            width="40"
                            height="40"
                            {}
                    }
            }
                     }
            }
            ul {
                li { (nav_button_with_class("Dashboard", "/dashboard", "outline")) }
            li{

                    (org_selection(orgs, current_name))

             }
            }


        }
    }
    .into_string();
    content::RawHtml(raw)
}

fn org_option(org: String) -> Markup {
    html! {
           option value=(org) { (org) }
    }
}

pub fn org_selection(orgs: Vec<Organization>, first: String) -> Markup {
    html! {
        select name="org" aria-label="Select" hx-post="/org_changed"  hx-swap="outerHTML" {
            option selected disabled value="" { (first) }
        @for org in orgs {
            (org_option(org.name().to_string()))
        }
        }
    }
}
