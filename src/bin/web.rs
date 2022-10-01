use yew::prelude::*;
use yew_router::prelude::*;

use comet::components::{
    footer::Footer,
    calculator::Calculator,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/simulator")]
    Simulator,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Simulator => html! { <Calculator/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
            <Footer/>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}
