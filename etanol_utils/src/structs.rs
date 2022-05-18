#[derive(Clone)]
pub struct Configs(pub Vec<(String, String)>);

impl Configs {
    pub fn new() -> Configs {
        Configs(Vec::new())
    }

    pub fn take(&self, key: String) -> Option<String> {
        for (k, v) in self.0.iter() {
            if k == &key {
                return Some(v.clone());
            }
        }

        None
    }
}
