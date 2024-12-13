use rocket::{form::Form, http::Header, response::content, State};

use crate::state::AppState;

use super::{nav::org_selection, OrgForm};

#[derive(Responder)]
pub struct HXResponder<T> {
    inner: T,
    header: Header<'static>,
}

#[post("/org_changed", data = "<org>")]
pub async fn changed(
    org: Form<OrgForm>,
    state: &State<AppState>,
) -> HXResponder<content::RawHtml<String>> {
    let org = org.into_inner().org;
    state.set_current_org(org.clone().into()).await;

    let orgs = state.fetch_organizations().await;

    let clone_orgs = orgs.clone();

    let o = org_selection(clone_orgs);

    HXResponder {
        inner: content::RawHtml(o.into_string()),
        header: HXHeader("org_changed".to_string()).into(),
    }
}

pub struct HXHeader(String);

impl From<HXHeader> for Header<'static> {
    #[inline(always)]
    fn from(hx: HXHeader) -> Self {
        Header::new("HX-Trigger", hx.0)
    }
}
