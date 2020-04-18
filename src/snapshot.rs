use url::Url;

pub struct Snapshot<'a> {
    base_url: &'a Url,
    instant: u64,
}

impl<'a> Snapshot<'a> {
    pub fn new(base_url: &'a Url, instant: u64) -> Self {
        Self { base_url, instant }
    }
}
