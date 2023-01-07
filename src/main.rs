mod app;
mod button;
mod card;
mod data;
mod debug;
mod drag;
mod event;
mod list;
mod prelude;
mod state;

use app::App;
use log::Level;

fn main() {
    let log_level = if cfg!(debug_assertions) {
        Level::Debug
    } else {
        Level::Warn
    };
    wasm_logger::init(wasm_logger::Config::new(log_level));
    yew::start_app::<App>();
}
