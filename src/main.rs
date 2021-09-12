use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wry::{
    application::{
        dpi::{LogicalSize, Size},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
    http::ResponseBuilder,
    webview::{RpcRequest, RpcResponse, WebViewBuilder},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct GreetParams {
    title: String,
}

fn main() {
    #[derive(RustEmbed)]
    #[folder = "dist"] // Path (relative to Carg.toml) with all the web resources.
    struct Assets;

    let event_loop = EventLoop::new();

    // Helps when debugging missing files.
    /* for file in Assets::iter() {
        println!("{}", file);
    } */

    // The window that hosts the webview.
    let window = WindowBuilder::new()
        .with_title("Wry React Typescript")
        .with_inner_size(Size::Logical(LogicalSize::new(1600f64, 900f64)))
        .with_resizable(true)
        .build(&event_loop)
        .expect("Failed to create window.");

    let handler = |window: &Window, mut req: RpcRequest| {
        let mut response = None;
        if &req.method == "open" {
            let path = std::env::current_dir().unwrap();
            let location = rfd::FileDialog::new()
                .add_filter("Text files (.txt)", &["txt"])
                .set_directory(&path)
                .pick_file()
                .expect("Failed to get file system location of data.");
            let content = std::fs::read_to_string(&location).expect("Failed to read file content.");
            response = Some(RpcResponse::new_result(
                req.id.take(),
                Some(json!(
                    {
                        "path": location,
                        "content": content
                    }
                )),
            ))
        } else if &req.method == "minimize" {
            window.set_minimized(true);
        } else if &req.method == "greet" {
            if let Some(params) = req.params.take() {
                let title = serde_json::from_value::<Vec<GreetParams>>(params)
                    .expect("Failed to parse parameters.");
                rfd::MessageDialog::new()
                    .set_title(title.first().expect("No param given.").title.as_str())
                    .show();
            }
        }

        response
    };

    let webview = WebViewBuilder::new(window)
        .expect("Failed to create webview.")
        .with_url("wry://")
        .expect("Failed to navigate to custom protocol.")
        .with_custom_protocol("wry".into(), move |request| {
            let url = request.uri();
            match url.replace("wry://", "").as_str() {
                // Load `index.html` if url is empty after removing the custom protocol name from it.
                "" | "/" => ResponseBuilder::new()
                .mimetype("text/html")
                .body(
                  Assets::get("index.html")
                        .expect("Did not find entry in folder.")
                        .into()
                ),
                file => {
                    // Remove the first char if its a slash, because we only want relative paths here.
                    let mut relative_file = file.to_string();
                    if relative_file.chars().next().unwrap_or('x') == '/' {
                        relative_file.remove(0);
                    }

                    // Return the file.
                    ResponseBuilder::new()
                    .mimetype(find_mimetype(relative_file.as_str()))
                    .body(
                      Assets::get(relative_file.as_str())
                        .unwrap_or_else(|| panic!("Did not find file: \"{}\" in React dist folder.", relative_file))
                        .into()
                    )
                }
            }
        })
        .with_rpc_handler(handler)
        .build()
        .expect("Failed to create webrenderer with custom protocol. \nIt is likely the Microsoft WebView2 Runtime is missing you can install it from here: https://developer.microsoft.com/en-us/microsoft-edge/webview2/");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                webview.resize().expect("Failed to resize webview.");
            }
            _ => (),
        }
    });
}

/// Returns the mimetype (or [MediaType](https://en.wikipedia.org/wiki/Media_type)) based on the file extension.
///
/// Remarks:
///
/// Only some mimetypes are supported, but the list can be expanded as needed.
///
/// Arguments:
///
/// * `path`: The path to the file for which the mime type is requested.
///
/// Returns:
///
/// The **mime type** as a `&str`.
fn find_mimetype(path: &str) -> &str {
    let parts: Vec<&str> = path.split('.').collect();

    match parts.last() {
        Some(v) => match *v {
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "svg" => "image/svg+xml",
            "json" => "application/json",
            "css" => "text/css",
            "js" => "text/javascript",
            &_ => "text/plain",
        },
        None => "text/plain",
    }
}
