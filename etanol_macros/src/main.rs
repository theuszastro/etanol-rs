use etanol_macros::AnswerFn;

#[derive(AnswerFn)]
struct User {
    name: String,
}

fn main() {
    assert_eq!(42, answer());
}
