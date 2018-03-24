
#[macro_use]
extern crate yew;
extern crate rand;

use yew::html::*;

struct Context {
}

struct Model {
}

enum Msg {
    Step,
}
const DIM: usize = 60;

fn update(_context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Step => {}
    };
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <header class="app-header",>
                <h1 class="app-title",>{ "Conway's Game of Life" }</h1>
            </header>
            <section class="game-container",>
                <section class="game-area",>
                    <table class="grid",>
                        { for (0..DIM).map(view_row) }
                    </table>
                    <div class="game-buttons",>
                        <button onclick=|_| Msg::Step,>{ "Next Generation" }</button>
                    </div>
                </section>
            </section>
        </div>
    }
}
fn view_row(_idx: usize) -> Html<Msg> {
    html! {
        <tr class="row",>
            { for (0..DIM).map(view_cell) }
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
    let context = Context {
    };
    let model = Model {
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}
