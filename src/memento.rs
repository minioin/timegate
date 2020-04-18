use crate::snapshot::Snapshot;
use url::Url;

pub struct Memento<'a> {
    base_url: &'a Url,
    snapshot: Snapshot<'a>,
    prev: Option<Snapshot<'a>>,
    next: Option<Snapshot<'a>>,
}

impl<'a> Memento<'a> {
    pub fn new(
        base_url: &'a Url,
        snapshot: Snapshot<'a>,
        prev: Option<Snapshot<'a>>,
        next: Option<Snapshot<'a>>,
    ) -> Self {
        Self {
            base_url,
            snapshot,
            prev,
            next,
        }
    }
}
