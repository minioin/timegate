use crate::map::TimeMap;
use crate::memento::Memento;
use crate::snapshot::Snapshot;
use crate::TimeGate;
use std::collections::{HashMap, HashSet};

use url::Url;

pub struct TimeGateServer {
    base_url: Url,
    db: sled::Db,
    snapshots: HashMap<Url, HashSet<u64>>,
}

impl TimeGate for TimeGateServer {
    fn map(&self, url: &Url) -> Option<TimeMap> {
        let tree = self.db.open_tree(url.as_str()).unwrap();
        let snapshots: HashSet<u64> = tree
            .scan_prefix(0u64.to_be_bytes())
            .keys()
            .map(|key| key.unwrap())
            .map(|key| to_u64(&key))
            .collect();

        Some(TimeMap::new(self.base_url.clone(), snapshots))
    }

    fn get(&self, url: &Url, timestamp: u64) -> Option<Memento<'_>> {
        if let Some(tree) = self.db.open_tree(url.as_str()).ok() {
            let snapshots: HashSet<u64> = tree
                .scan_prefix(timestamp.to_be_bytes())
                .keys()
                .take(1)
                .map(|key| key.unwrap())
                .map(|key| to_u64(&key))
                .collect();
            if let Some(snapshot) = snapshots.get(&0) {
                Some(Memento::new(
                    &self.base_url,
                    Snapshot::new(&self.base_url, *snapshot),
                    None,
                    None,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl TimeGateServer {
    fn select(&self, _: u64) -> u64 {
        // TODO: Do a binary search on hashset to select best instant.
        0
    }
}

fn to_u64(values: &[u8]) -> u64 {
    values.iter().take(8).fold(0, |x, &i| x << 8 | i as u64)
}
