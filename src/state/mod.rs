use std::sync::Arc;

use rocket::tokio::sync::RwLock;
use rocket::{Build, Rocket};

use crate::{github::organization::Organization, remote};

pub struct GithubState {
    organizations: Vec<Organization>,
}

impl GithubState {
    pub async fn fetch_organizations(&mut self) -> Vec<Organization> {
        let orgs = remote::get_organizations().await;
        self.organizations = orgs;
        self.organizations.clone()
    }
}

pub struct AppState {
    pub github: Arc<RwLock<GithubState>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            github: Arc::new(RwLock::new(GithubState {
                organizations: vec![],
            })),
        }
    }

    pub async fn fetch_organizations(&self) -> Vec<Organization> {
        self.github.write().await.fetch_organizations().await
    }
}

pub fn mount_state(rocket: Rocket<Build>) -> Rocket<Build> {
    let state = AppState::new();

    rocket.manage(state)
}
