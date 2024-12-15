use crate::github::{organization::Organization, repo::Repo};

pub async fn auth(token: String) {
    let builder = octocrab::Octocrab::builder();
    octocrab::initialise(builder.personal_token(token).build().unwrap());
}

pub async fn get_organizations() -> Vec<Organization> {
    let octocrab = octocrab::instance();

    let orgs = octocrab
        .current()
        .list_org_memberships_for_authenticated_user()
        .send()
        .await
        .unwrap();

    let mut organizations = vec![];

    orgs.items.iter().for_each(|o| {
        organizations.push(Organization::new(o.organization.login.clone().as_str()));
    });

    organizations
}

pub async fn get_repos(org: Organization) -> Vec<Repo> {
    let octocrab = octocrab::instance();
    let page: u32 = 0;
    let max: u8 = 100;
    let repos = octocrab
        .orgs(org.name().clone())
        .list_repos()
        .page(page)
        .per_page(max)
        .send()
        .await
        .unwrap();

    let mut state_repos: Vec<Repo> = vec![];
    for repo in repos.items.iter() {
        state_repos.push(Repo::new(&repo.name));
    }

    state_repos
}
