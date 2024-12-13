use std::collections::HashMap;
use std::sync::Arc;

use rocket::tokio::sync::RwLock;
use rocket::{Build, Rocket};

use crate::github::repo::Repo;
use crate::{github::organization::Organization, remote};

pub struct GithubState {
    organizations: Vec<Organization>,
    repos: HashMap<Organization, Vec<Repo>>,
}

impl GithubState {
    pub async fn fetch_organizations(&mut self) -> Vec<Organization> {
        let orgs = remote::get_organizations().await;
        self.organizations = orgs;
        self.organizations.clone()
    }

    pub async fn fetch_repos(&mut self, owner: Organization) -> Vec<Repo> {
        let repos = remote::get_repos(owner.clone()).await;
        self.repos.insert(owner.clone(), repos);
        self.repos.get(&owner).cloned().unwrap_or_default()
    }
}

pub struct AppState {
    pub github: Arc<RwLock<GithubState>>,
    pub current_org: Arc<RwLock<Option<Organization>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            github: Arc::new(RwLock::new(GithubState {
                organizations: vec![],
                repos: HashMap::new(),
            })),
            current_org: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn fetch_organizations(&self) -> Vec<Organization> {
        self.github.write().await.fetch_organizations().await
    }

    pub async fn fetch_repos(&self, owner: Organization) -> Vec<Repo> {
        self.github.write().await.fetch_repos(owner).await
    }

    pub async fn set_current_org(&self, org: Organization) {
        *self.current_org.write().await = Some(org);
    }

    pub async fn current_org(&self) -> Option<Organization> {
        self.current_org.read().await.clone()
    }
}

pub fn mount_state(rocket: Rocket<Build>) -> Rocket<Build> {
    let state = AppState::new();

    rocket.manage(state)
}
