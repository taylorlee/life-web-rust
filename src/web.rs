
use std::time::Duration;

use yew;
use yew::html::*;
use yew::services::interval::IntervalService;
use yew::services::Task;

use game::*;

struct Model {
    board: Board,
    clock: usize,
    dim: usize,
    job: Option<Box<Task>>,
}

enum Msg {
    Step,
    Incr,
    Decr,
    Start,
    Stop,
    Reset,
}

struct Context {
    interval: IntervalService<Msg>,
}

const DIM: usize = 10;
const FLICK: u64 = 100;

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Start => {
            let handle = context.interval.spawn(Duration::from_millis(FLICK), || Msg::Step);
            model.job = Some(Box::new(handle));
        },
        Msg::Stop => {
            if let Some(mut task) = model.job.take() {
                task.cancel();
            }
            model.job = None;
        },
        Msg::Reset => {
            model.board = setup();
            model.clock = 0;
        },
        Msg::Step => {
            model.clock += 1;
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
                            <button class="game-button", onclick=move|_| Msg::Start,>{ "Start" }</button>
                            <button class="game-button", onclick=move|_| Msg::Stop,>{ "Stop" }</button>
                            <button class="game-button", onclick=move|_| Msg::Reset,>{ "Reset" }</button>
                            <button class="button",  onclick=|_| Msg::Incr,>{ "Zoom In" }</button>
                            <button class="button", onclick=|_| Msg::Decr,>{ "Zoom Out" }</button>
                            <span class="tag",> {"CLOCK"} { model.clock } </span>
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
pub fn serve() {
    yew::initialize();
    let mut app = App::new();
    let context = Context {
        interval: IntervalService::new(app.sender()),
    };
    let model = Model {
        board: setup(),
        dim: 20,
        clock: 0,
        job: None,
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}
