use wry::{Application, Attributes, Result};

fn main() -> Result<()> {
    let mut app = Application::new()?;
    let window = app.add_window(Attributes{
        title: String::from("Hello React"),
        html: Some(include_str!("../dist/index.html").to_string()),
        ..Default::default()
    })?;
    window.set_always_on_top(true).expect("Failed to set always on top for window.");
    app.run();
    Ok(())
}