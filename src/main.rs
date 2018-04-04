#![recursion_limit = "256"]

mod game;
mod model;
mod actions;
mod view;

extern crate rand;
#[macro_use]
extern crate yew;

use yew::{initialize, run_loop, html::App, services::interval::IntervalService};

fn main() {
    initialize();
    let mut app = App::new();
    let context = model::Context {
        interval: IntervalService::new(app.sender()),
    };
    let model = model::Model {
        board: game::setup(),
        speed: 5,
        clock: 0,
        job: None,
        running: false,
    };
    app.mount(context, model, actions::update, view::render);
    run_loop();
}
