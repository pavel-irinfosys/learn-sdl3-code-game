mod window;

use window::Window;


fn main() {
    let window = Window::new("Hi All", 800, 640);
    window.view();
}
