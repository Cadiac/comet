use gloo_worker::{Spawnable, WorkerBridge};
use log::debug;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::game::{GameResult, Outcome};
use crate::simulator::{Cmd, Simulator, Status};

#[derive(Debug)]
pub enum Msg {
    ChangeSimulationsCount(usize),
    ChangeAdvantage(usize),
    ChangeLoyalty(i32),
    ChangeDamage(u32),
    ToggleSquirrels,
    BeginSimulation,
    CancelSimulation,
    UpdateProgress(usize, usize, Vec<GameResult>),
    FinishSimulation(usize, usize, Vec<GameResult>),
    SimulationError(String),
    DismissError,
}

#[derive(Debug, Default)]
struct Results {
    wins: u32,
    losses: u32,
    win_percentage: f32,

    total_damage: u32,
    avg_damage: f32,

    total_squirrels: u32,
    avg_squirrels: f32,

    total_returns: u32,
    avg_returns: f32,

    total_rolls: u32,
    avg_rolls: f32,
}

pub struct Calculator {
    /// Total simulations to simulate
    simulations: usize,

    /// Should combat damage using squirrels be included
    squirrels: bool,

    /// Extra roll advantage effects
    advantage: usize,

    /// Starting loyalty of the planeswalker
    loyalty: i32,

    /// Damage required to win for win
    damage: u32,

    /// Error message from simulation
    error_msg: Option<String>,

    /// Simulation progress
    progress: (usize, usize),

    /// Simulation results
    results: Results,

    /// Web worker running the simulations
    worker: WorkerBridge<Simulator>,

    /// Is the simulator busy running
    is_busy: bool,
}

impl Calculator {
    fn update_results(&mut self, new_results: Vec<GameResult>) {
        for GameResult {
            outcome,
            damage,
            squirrels,
            rolls,
            returns,
        } in new_results.into_iter()
        {
            match outcome {
                Outcome::Win => self.results.wins += 1,
                Outcome::Lose => self.results.losses += 1,
            }

            self.results.total_damage += damage;
            self.results.total_squirrels += squirrels;
            self.results.total_returns += returns as u32;
            self.results.total_rolls += rolls as u32;
        }

        let total_simulations = u32::max(self.results.wins + self.results.losses, 1) as f32;

        self.results.win_percentage = 100.0 * self.results.wins as f32 / total_simulations;
        self.results.avg_returns = self.results.total_returns as f32 / total_simulations;
        self.results.avg_damage = self.results.total_damage as f32 / total_simulations;
        self.results.avg_rolls = self.results.total_rolls as f32 / total_simulations;
        self.results.avg_squirrels = self.results.total_squirrels as f32 / total_simulations;
    }
}

impl Component for Calculator {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        let worker = Simulator::spawner()
            .callback(move |results| {
                match results {
                    Status::InProgress(current, total, results) => {
                        link.send_message(Msg::UpdateProgress(current, total, results))
                    }
                    Status::Cancelled(current, total) => {
                        link.send_message(Msg::FinishSimulation(current, total, Vec::new()))
                    }
                    Status::Complete(total, results) => {
                        link.send_message(Msg::FinishSimulation(total, total, results))
                    }
                    Status::Error(message) => link.send_message(Msg::SimulationError(message)),
                };
            })
            .spawn("/worker.js");

        Self {
            simulations: 1000000,
            squirrels: true,
            advantage: 0,
            loyalty: 5,
            damage: 20,
            progress: (0, 0),
            results: Results::default(),
            error_msg: None,
            worker,
            is_busy: false,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _: &Context<Self>) {}

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        debug!("[Update]: {msg:?}");

        match msg {
            Msg::ChangeAdvantage(advantage) => {
                self.advantage = advantage;
            }
            Msg::ChangeSimulationsCount(simulations) => {
                self.simulations = simulations;
            }
            Msg::ChangeLoyalty(loyalty) => {
                self.loyalty = loyalty;
            }
            Msg::ChangeDamage(damage_target) => {
                self.damage = damage_target;
            }
            Msg::ToggleSquirrels => {
                self.squirrels = !self.squirrels;
            }
            Msg::BeginSimulation => {
                self.is_busy = true;
                self.error_msg = None;
                self.results = Results::default();

                self.worker.send(Cmd::Begin {
                    simulations: self.simulations,
                    squirrels: self.squirrels,
                    advantage: self.advantage,
                    loyalty: self.loyalty,
                    damage: self.damage,
                });
            }
            Msg::CancelSimulation => {
                self.worker.send(Cmd::Cancel);
            }
            Msg::UpdateProgress(progress, total_simulations, results) => {
                self.progress = (progress, total_simulations);
                self.update_results(results);
            }
            Msg::FinishSimulation(progress, total_simulations, results) => {
                self.progress = (progress, total_simulations);
                self.is_busy = false;
                self.update_results(results);
            }
            Msg::SimulationError(message) => {
                self.is_busy = false;
                self.error_msg = Some(message);
            }
            Msg::DismissError => self.error_msg = None,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let is_ready = !self.is_busy && self.simulations > 0;
        let (progress, total_games) = self.progress;

        return html! {
            <main class="container">
                <article class="main-container">
                    <header>
                        <h1 class="title">{ "Comet, Stellar Pup Simulator 🐶" }</h1>
                    </header>

                    <div class="grid">
                        <div class="card-container">
                            <img class="card" src="comet.jpeg"/>
                        </div>
                        <div>
                            <div class="grid">
                                <label class="label" for="simulated-games">
                                    {"Starting loyalty:"}
                                    <input class="input is-info" type="number" id="starting-loyalty" step="1" min="1" value={self.loyalty.to_string()}
                                        onchange={link.batch_callback(move |e: Event| {
                                            let target: Option<EventTarget> = e.target();
                                            let select = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                            select.map(|select| {
                                                let count = select.value();
                                                Msg::ChangeLoyalty(count.parse().unwrap_or(5))
                                            })
                                        })}
                                    />
                                </label>

                                <label class="label" for="simulated-games">
                                    {"Opponent health:"}
                                    <input class="input is-info" type="number" id="damage" step="1" min="1" value={self.damage.to_string()}
                                        onchange={link.batch_callback(move |e: Event| {
                                            let target: Option<EventTarget> = e.target();
                                            let select = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                            select.map(|select| {
                                                let count = select.value();
                                                Msg::ChangeDamage(count.parse().unwrap_or(20))
                                            })
                                        })}
                                    />
                                </label>
                            </div>

                            <label class="label" for="roll-advantage">
                                {"Extra roll advantage effects:"}
                                <input class="input is-info" type="number" id="roll-advantage" step="1" min="1" value={self.advantage.to_string()}
                                    onchange={link.batch_callback(move |e: Event| {
                                        let target: Option<EventTarget> = e.target();
                                        let select = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                        select.map(|select| {
                                            let count = select.value();
                                            Msg::ChangeAdvantage(count.parse().unwrap_or(0))
                                        })
                                    })}
                                />
                            </label>

                            <label class="label" for="simulated-games">
                                {"Games to simulate:"}
                                <input class="input is-info" type="number" id="simulated-games" step="1000" min="0" value={self.simulations.to_string()}
                                    onchange={link.batch_callback(move |e: Event| {
                                        let target: Option<EventTarget> = e.target();
                                        let select = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                        select.map(|select| {
                                            let count = select.value();
                                            Msg::ChangeSimulationsCount(count.parse().unwrap_or(10000))
                                        })
                                    })}
                                />
                            </label>

                            <label for="squirrels">
                                <input type="checkbox" id="squirrels" checked={self.squirrels} onchange={link.callback(|_| Msg::ToggleSquirrels)}/>
                                {"Include damage from squirrels"}
                            </label>

                            <div class="buttons">
                                <div class={if is_ready { "primary" } else { "primary outline" }}
                                    type="submit"
                                    role="button"
                                    disabled={!is_ready}
                                    aria-busy={if !is_ready { "true" } else { "" }}
                                    onclick={link.callback(|_| Msg::BeginSimulation)}>
                                    { "Run simulation ▶︎" }
                                </div>

                                <div role="button" disabled={!self.is_busy} onclick={link.callback(|_| Msg::CancelSimulation)}>
                                    { "Cancel" }
                                </div>
                            </div>
                        </div>
                    </div>

                    <footer>
                        <div class="progress-container">
                            <label>{"Progress:"}</label>
                            <span>{format!("{progress}/{total_games}")}</span>
                            <progress class="progress primary" value={progress.to_string()} max={total_games.to_string()}>
                                { format!("{progress}/{total_games}") }
                            </progress>
                        </div>

                        <div>
                            <label>{"Results:"}</label>
                            <figure>
                                <table>
                                    <thead>
                                        <tr>
                                            <th><abbr title="Wins">{"Wins"}</abbr></th>
                                            <th><abbr title="Losses">{"Losses"}</abbr></th>
                                            <th><abbr title="Win percentage">{"Win %"}</abbr></th>
                                            <th><abbr title="Average damage">{"Damage"}</abbr></th>
                                            <th><abbr title="Average number of squirrels produced">{"Squirrels"}</abbr></th>
                                            <th><abbr title="Average number of cards returned from graveyard">{"Returns"}</abbr></th>
                                            <th><abbr title="Average total rolls">{"Rolls"}</abbr></th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td>{self.results.wins}</td>
                                            <td>{self.results.losses}</td>
                                            <td>{format!("{:.2}%", self.results.win_percentage)}</td>
                                            <td>{format!("{:.2}", self.results.avg_damage)}</td>
                                            <td>{format!("{:.2}", self.results.avg_squirrels)}</td>
                                            <td>{format!("{:.2}", self.results.avg_returns)}</td>
                                            <td>{format!("{:.2}", self.results.avg_rolls)}</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </figure>
                        </div>
                    </footer>
                </article>
            </main>
        };
    }
}
