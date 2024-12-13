use maud::html;
use rocket::{response::content, State};

use crate::{github::repo::Repo, state::AppState};

#[post("/dashboard")]
pub async fn get(state: &State<AppState>) -> content::RawHtml<String> {
    let current = state.current_org().await;

    let (repos, o) = match current {
        Some(o) => (state.fetch_repos(o.clone()).await, o),
        None => (vec![], "".to_string().into()),
    };

    content::RawHtml(dashboard(repos, o.name().to_owned()))
}

pub fn dashboard(repos: Vec<Repo>, org: String) -> String {
    html! {
        div hx-post="/dashboard" hx-trigger="org_changed from:body" hx-swap="outerHTML"{
            article   {

                header {
                    h2 { "Dashboard" }
                    p { (org) }

            }

            body{
                 div {
                         // GitHub
                         h3 { "Repos" }
                           ul {
                               @for repo in repos {
                                   li { (repo.name()) }
                               }
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
