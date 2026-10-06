mod window;
mod utility;

use window::Window;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut win = Window::new("My App", 800, 640)?;
    win.view();
    Ok(())
}
