use yew::html::*;
//use std::iter;
use game::*;
use yew;

pub fn serve() {
    yew::initialize();
    let mut app = App::new();
    let context = Context {};
    let model = Model { 
        board: setup(),
        dim: 20,
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}

struct Context {}

struct Model {
    board: Board,
    dim: usize,
}

enum Msg {
    Step,
    Incr,
    Decr,
}

const DIM: usize = 50;

fn update(_context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Step => {
            model.board = next_generation(&model.board);
        }
        Msg::Incr => {
            model.dim += 1;
        }
        Msg::Decr => {
            model.dim -= 1;
        }
    };
}

fn view(model: &Model) -> Html<Msg> {
    let mut rows = [[false; DIM]; DIM];
    for rdx in 0..DIM {
        for cdx in 0..DIM {
            if model.board.contains(&(rdx as isize, cdx as isize)) {
                rows[rdx][cdx] = true;
            }
        }
    }

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
                                { for rows.iter().map(view_row)  }
                            </table>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

fn view_row(cells: &[bool; DIM]) -> Html<Msg> {
    html! {
        <tr class="row",>
            { for cells.iter().map(view_cell) }
        </tr>
    }
}

fn view_cell(living: &bool) -> Html<Msg> {
    html! {
        <td class=("cell", if *living { "living" } else { "dead" } ),</td>
    }
}

