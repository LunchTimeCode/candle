use maud::html;
use rocket::{response::content, State};

use crate::state::AppState;

#[get("/dashboard")]
pub async fn get(state: &State<AppState>) -> content::RawHtml<String> {
    let orgs = state.fetch_organizations().await;

    let raw = html! {


     article  {

         header {
             h2 { "Dashboard" }

     }

     body{
          div {
                  // GitHub
                  h3 { "Organizations" }
                    ul {
                        @for org in orgs {
                            li { (org.name()) }
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
    .into_string();
    content::RawHtml(raw)
}
