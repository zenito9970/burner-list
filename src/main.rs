mod app;
mod button;
mod card;
mod data;
mod debug;
mod drag;
mod event;
mod list;
mod state;
mod prelude;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
