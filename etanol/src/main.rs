use etanol::{Model, User};

fn main() {
    let mut user = User::new("1".to_string(), "John".to_string());

    user.skip(10).take(10).execute();
}
