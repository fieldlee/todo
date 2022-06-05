mod pages;
mod router;
mod components;
mod store;
mod api;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Route,switch};
use crate::components::orgnanisms::navbar::NavBar;

#[function_component(App)]
pub fn app() -> Html{
    html!{
        <BrowserRouter>
            <NavBar />
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}