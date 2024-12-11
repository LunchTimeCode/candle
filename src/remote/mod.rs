use crate::github::organization::Organization;

pub async fn get_organizations() -> Vec<Organization> {
    let organizations = vec![
        Organization::new("rust-lang"),
        Organization::new("rust-lang-nursery"),
    ];
    organizations
}
