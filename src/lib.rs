use crate::map::TimeMap;
use crate::memento::Memento;
use url::Url;

mod gate;
mod map;
mod memento;
mod snapshot;

pub trait TimeGate {
    /// Get pure timemap for a Url resource. The response does not contain any extra header.
    fn map(&self, url: &Url) -> Option<TimeMap>;

    /// Accept-Datetime based negotiation endpoint that returns the memento. Contains extra headers
    /// that contain extra information like gate and map url, previous and next versions,
    /// etc.
    fn get(&self, url: &Url, timestamp: u64) -> Option<Memento>;
}

pub trait SelectionStrategy {
    fn select(&self, timestamp: u64) -> u64;
}
