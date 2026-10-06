use super::Window;

impl Window {
    pub fn view(&self) {
        println!("Window '{}' ({}x{})", self.title, self.width, self.height);
    }
}
