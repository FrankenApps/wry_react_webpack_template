use rust_embed::RustEmbed;
use wry::{Application, Attributes, CustomProtocol, Result};

fn main() -> Result<()> {
  #[derive(RustEmbed)]
  #[folder = "dist"] // Path (relative to Carg.toml) with all the web resources.
  struct Assets;

  let mut app = Application::new()?;

  app.add_window_with_configs(
    Attributes {
        title: String::from("Hello React"),
        url: Some("wry://".to_string()), // navigate to your custom protocol
        ..Default::default()
    },
    None,
    vec![CustomProtocol {
      name: "wry".into(),
      handler: Box::new(|url| {
        match url.replace("wry://", "").as_str() {
          // load index.html if url is empty after removing your custom protocol name from it
          "" => Ok(Assets::get("index.html").unwrap().into()),
          file => Ok(Assets::get(file).unwrap().into()),
        }
      }),
    }],
    None,
  )?;
  app.run();
  Ok(())
}