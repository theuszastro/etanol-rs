pub struct Create {
    values: Vec<(String, String)>,
    result: bool,
}

pub trait Model {
    fn create() -> &'static mut Create;
}
