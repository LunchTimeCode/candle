use maud::html;
use rocket::response::content;

#[get("/dashboard")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {


     article  {

         header {
             h2 { "Dashboard" }

     }

         }

         body{
              div {
                      // GitHub
                      h3 { "GitHub" }
              }
         }


         footer {
              p{
                  "This is a work in progress."
              }
         }

    }
    .into_string();
    content::RawHtml(raw)
}
