use gloo_worker::{HandlerId, Worker, WorkerScope};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::{Arc, Mutex};

use js_sys::Promise;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::WorkerGlobalScope;

use crate::game::{Game, GameResult};

const MAX_BATCH_SIZE: usize = 10000;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Cmd {
    Begin {
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
    },
    Cancel,
}

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Running,
    Cancelling,
}

#[derive(Debug)]
pub enum Msg {
    Command { cmd: Cmd, id: HandlerId },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    InProgress(usize, usize, Vec<GameResult>),
    Cancelled(usize, usize),
    Complete(usize, Vec<GameResult>),
    Error(String),
}

/// Yields execution from worker by creating a `setTimeout(0)` with `WorkerGlobalScope`
/// This is necessary because worker has no access to `window`.
/// Modified from: extraymond @ https://extraymond.github.io/posts/2019-08-25-let-s-create-a-task-manager-in-webworker/
pub async fn yield_now() {
    let promise = Promise::new(&mut |yes, _| {
        let global = js_sys::global();
        let scope = global.dyn_into::<WorkerGlobalScope>().unwrap();
        scope
            .set_timeout_with_callback_and_timeout_and_arguments_0(&yes, 0)
            .unwrap();
    });
    let js_fut = JsFuture::from(promise);
    js_fut.await.unwrap();
}

pub struct Simulator {
    state: Arc<Mutex<State>>,
}

impl Simulator {
    async fn run(
        state: Arc<Mutex<State>>,
        scope: WorkerScope<Self>,
        id: HandlerId,
        simulations: usize,
        squirrels: bool,
        advantage: usize,
        loyalty: i32,
        damage: u32,
    ) {
        {
            let mut state = state.lock().unwrap();
            if *state != State::Idle {
                return;
            }

            *state = State::Running;
        }

        let mut progress = 0;
        scope.respond(id, Status::InProgress(progress, simulations, Vec::new()));

        loop {
            if progress >= simulations {
                break;
            }

            if State::Cancelling == *state.lock().unwrap() {
                scope.respond(id, Status::Cancelled(progress, simulations));
                break;
            }

            yield_now().await;

            let batch_size = if progress + MAX_BATCH_SIZE > simulations {
                simulations - progress
            } else {
                MAX_BATCH_SIZE
            };

            progress += batch_size;

            match Simulator::run_batch(batch_size, squirrels, advantage, loyalty, damage) {
                Ok(results) => {
                    if progress == simulations {
                        scope.respond(id, Status::Complete(simulations, results));
                    } else {
                        scope.respond(id, Status::InProgress(progress, simulations, results));
                    }
                }
                Err(err) => {
                    scope.respond(
                        id,
                        Status::Error(format!("failed to simulate simulations: {err:?}")),
                    );
                }
            }
        }

        *state.lock().unwrap() = State::Idle;
    }

    fn run_batch(
        batch_size: usize,
        squirrels: bool,
        advantage: usize,
        loyalty: i32,
        damage: u32,
    ) -> Result<Vec<GameResult>, Box<dyn Error>> {
        let mut results = Vec::new();

        for _ in 0..batch_size {
            let mut game = Game::new(squirrels, advantage, loyalty, damage);
            let result = game.run();
            results.push(result);
        }

        Ok(results)
    }
}

impl Worker for Simulator {
    type Input = Cmd;

    type Message = Msg;

    type Output = Status;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::Idle)),
        }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        match msg {
            Msg::Command { cmd, id } => match cmd {
                Cmd::Begin {
                    simulations,
                    squirrels,
                    advantage,
                    loyalty,
                    damage,
                } => {
                    let (state, scope) = (Arc::clone(&self.state), scope.clone());

                    spawn_local(async move {
                        Simulator::run(
                            state, scope, id, simulations, squirrels, advantage, loyalty, damage,
                        )
                        .await;
                    });
                }
                Cmd::Cancel => {
                    let mut state = self.state.lock().unwrap();
                    *state = State::Cancelling;
                }
            },
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        scope.send_message(Msg::Command { cmd: msg, id })
    }
}
