use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;

mod routing;
mod pages;

use pages::{blog::Blog, home::Home, notfound::NotFound};
use routing::AppRoute;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &AppRoute) -> Html {
    log::info!("{:?}", "tst");
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Blog => html! { <Blog /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[wasm_bindgen]
pub fn start(mode: AppMode) {
    let log_level = match mode {
        AppMode::Dev => log::Level::Trace,
        AppMode::Production => log::Level::Error,
    };
    wasm_logger::init(wasm_logger::Config::new(log_level));
    yew::start_app::<Main>();
}

pub fn main() {
    panic!()
}

#[wasm_bindgen]
pub enum AppMode {
    Dev,
    Production,
}