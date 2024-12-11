use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod github;
mod remote;
mod state;
mod view;

#[launch]
fn rocket() -> _ {
    let port = 12500;
    let address = "0.0.0.0";

    let config = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", address));
    let rocket = rocket::custom(config);

    webbrowser::open(&format!("http://{}:{}", address, port)).unwrap_or_default();
    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let with_index = rocket.mount(
        "/",
        routes![view::index, view::nav::get, view::dashboard::get,],
    );

    let rocket = assets::mount_assets(with_index);
    state::mount_state(rocket)
}
