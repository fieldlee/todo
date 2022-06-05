use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::create_account::CreateAccount;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::login::Login;

#[derive(Routable,Clone,PartialEq)]
pub enum Route{
    #[at("/")]
    Home,
    #[at("/create_account")]
    CreateAccount,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::CreateAccount => html! { <CreateAccount /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Login => html! { <Login /> },
    }
}