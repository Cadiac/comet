use log::debug;
use yew::prelude::*;

use crate::game::{Game};

#[derive(Debug)]
pub enum Msg {
    Activate,
    Reset,
}

pub struct Playground {
    /// The Comet planeswalker simulation
    game: Game,
}

impl Component for Playground {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: Game::new(true, 3, 5, 20),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _: &Context<Self>) {}

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        debug!("[Update]: {msg:?}");

        match msg {
            Msg::Activate => {
                self.game.activate();
            },
            Msg::Reset => {
                self.game = Game::new(true, 0, 5, 20);
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        return html! {
            <main class="container centered">
                <div class="relative-container">
                    <img src="comet-blank.jpeg"
                        class="card"
                        onclick={link.callback(|_| Msg::Activate)}
                    />
                    <span class="loyalty">{self.game.loyalty}</span>
                </div>
            </main>
        };
    }
}
