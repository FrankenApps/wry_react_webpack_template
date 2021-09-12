# wry_react_webpack_template
A template to demonstrate the use of [tauri-apps/wry](https://github.com/tauri-apps/wry) with React and Webpack in order to create self-contained single binary apps.

Binaries for all platforms can be built using github actions.

### How to run on local machine
1. Install dependencies: `npm install`
2. Build frontend UI: `npm run build`
3. Serve files using _wry_: `cargo run --release`