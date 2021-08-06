#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]


use game_lib::setup_app;

fn main() {
    let mut app = setup_app();
    app.run();
}
