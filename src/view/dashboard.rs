use super::components;
use crate::{github::repo::Repo, state::AppState};
use maud::html;
use rocket::{response::content, State};

#[post("/dashboard")]
pub async fn get(state: &State<AppState>) -> content::RawHtml<String> {
    let current = state.current_org().await;
    let orgs = state.fetch_organizations().await;
    let first = orgs.first().cloned().unwrap();

    let (repos, o) = match current {
        Some(o) => (state.fetch_repos(o.clone()).await, o),
        None => (state.fetch_repos(first.clone()).await, first),
    };

    content::RawHtml(dashboard(repos, o.name().to_owned()))
}

pub fn dashboard(repos: Vec<Repo>, org: String) -> String {
    let headers = vec!["Name"];

    html! {
        div hx-post="/dashboard" hx-trigger="org_changed from:body" hx-swap="outerHTML"  {

            article   {

                header {
                    h2 { "Dashboard" }
                    p { (org) }

            }

            body{
                 div  {
                         // GitHub
                         h3 { "Repos" }
                               (components::generate_header(headers.clone()))
                               div ."overflow-auto" ."fixed-container" {

                               (components::generate_table(repos, headers))
                               }

                 }


            }


            footer {
                 p{
                     "This is a work in progress."
                 }
            }
                }
        }

    }
    .into_string()
}
