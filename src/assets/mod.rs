use rocket::{response::content, Build, Rocket};

#[get("/pico.violet.min.css")]
fn pico_code() -> content::RawCss<&'static str> {
    let pico = include_str!("../../assets/pico.violet.min.css");
    content::RawCss(pico)
}

#[get("/app.css")]
fn app_css() -> content::RawCss<&'static str> {
    let app = include_str!("../../assets/app.css");
    content::RawCss(app)
}

#[get("/htmx.min.js")]
fn htmx_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../../assets/htmx.min.js");
    content::RawJavaScript(app)
}

#[get("/pico_ext.css")]
fn pico_ext_css() -> content::RawCss<&'static str> {
    let app = include_str!("../../assets/pico_ext.css");
    content::RawCss(app)
}

pub fn mount_assets(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/_assets",
        routes![htmx_code, app_css, pico_code, pico_ext_css,],
    )
}
