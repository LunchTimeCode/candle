use crate::github::{organization::Organization, repo::Repo};

pub async fn get_organizations() -> Vec<Organization> {
    let organizations = vec![
        Organization::new("LunchTimeCode"),
        Organization::new("SilenLoc"),
    ];
    organizations
}

pub async fn get_repos(org: Organization) -> Vec<Repo> {
    let repos1 = vec![Repo::new("candle"), Repo::new("me")];
    let repos2 = vec![Repo::new("super"), Repo::new("cool")];

    if org.name() == "SilenLoc" {
        repos1
    } else {
        repos2
    }
}
