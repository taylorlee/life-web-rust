#![recursion_limit = "128"]

extern crate rand;
#[macro_use]
extern crate yew;

use yew::html::*;
use std::iter;

struct Context {}

struct Model {
    dim: usize,
}

enum Msg {
    Step,
    Incr,
    Decr,
}

fn update(_context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Step => {}
        Msg::Incr => {
            model.dim += 1;
        }
        Msg::Decr => {
            model.dim -= 1;
        }
    };
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <header class="app-header",>
                <section class="section",>
                    <div class="container",>
                        <h1 class="title",>{ "Conway's Game of Life" }</h1>
                    </div>
                </section>
            </header>
            <section class="section",>
                <div class="container",>
                    <div class="level",>
                        <div class="level-item",>
                            <button class="button", onclick=|_| Msg::Step,>{ "Next Generation" }</button>
                            <button class="button",  onclick=|_| Msg::Incr,>{ "Zoom In" }</button>
                            <span class="tag",> { model.dim } </span>
                            <button class="button", onclick=|_| Msg::Decr,>{ "Zoom Out" }</button>
                        </div>
                    </div>
                </div>
            </section>
            <section class="section",>
                <div class="container",>
                    <div class="level",>
                        <div class="level-item",>
                            <table class="grid",>
                                { for (0..model.dim).zip(iter::repeat(model.dim)).map(view_row) }
                            </table>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

fn view_row((_idx, dim): (usize, usize)) -> Html<Msg> {
    html! {
        <tr class="row",>
            { for (0..dim).map(view_cell) }
        </tr>
    }
}

fn view_cell(_idx: usize) -> Html<Msg> {
    html! {
        <td class=("cell", if rand::random() { "living" } else { "dead" } ),</td>
    }
}

fn main() {
    yew::initialize();
    let mut app = App::new();
    let context = Context {};
    let model = Model { dim: 20 };
    app.mount(context, model, update, view);
    yew::run_loop();
}
