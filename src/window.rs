mod render;

pub struct Window {
    title: String,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self { title: title.to_string(), width, height }
    }
}
