
use std::collections::HashSet;
use url::Url;

pub struct TimeMap {
    base_url: Url,
    snapshots: HashSet<u64>,
}

impl TimeMap {
    pub fn new(base_url: Url, snapshots: HashSet<u64>) -> Self {
        Self {
            base_url,
            snapshots,
        }
    }
}
