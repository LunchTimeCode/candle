use maud::html;
use rocket::response::content;

use crate::view::components::nav_button_with_class;

#[get("/nav")]
pub fn get() -> content::RawHtml<String> {
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
            }
        }
    }
    .into_string();
    content::RawHtml(raw)
}
