use std::env;

pub fn findEnv(name: String) -> Option<String> {
    for (key, value) in env::vars() {
        if key == name {
            return Some(value);
        }
    }

    None
}
