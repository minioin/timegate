use crate::map::TimeMap;
use crate::memento::Memento;
use crate::snapshot::Snapshot;
use crate::{SelectionStrategy, TimeGate};
use std::collections::{HashMap, HashSet};
use url::Url;

pub struct InMemoryGateServer {
    base_url: Url,
    snapshots: HashMap<Url, HashSet<u64>>,
}

impl TimeGate for InMemoryGateServer {
    fn map(&self, url: &Url) -> Option<TimeMap> {
        match self.snapshots.get(url) {
            Some(snapshots) => Some(TimeMap::new(self.base_url.clone(), snapshots.clone())),
            _ => None,
        }
    }

    fn get(&self, url: &Url, timestamp: u64) -> Option<Memento<'_>> {
        if let Some(snapshots) = self.snapshots.get(url) {
            let index = self.select(timestamp);

            let snapshot = snapshots
                .get(&index)
                .map(|s| Snapshot::new(&self.base_url, *s))
                .unwrap();
            let next = snapshots
                .get(&(index + 1))
                .map(|s| Snapshot::new(&self.base_url, *s));
            let prev = snapshots
                .get(&(index - 1))
                .map(|s| Snapshot::new(&self.base_url, *s));

            return Some(Memento::new(&self.base_url, snapshot, prev, next));
        }
        None
    }
}

impl SelectionStrategy for InMemoryGateServer {
    fn select(&self, _: u64) -> u64 {
        // TODO: Do a binary search on hashset to select best instant.
        0
    }
}
