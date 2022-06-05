use yew::prelude::*;
use stylist::yew::styled_component;
use crate::components::molecules::account_form::{AccountForm,User,Action};
use crate::api::login;
use yewdux::prelude::PersistentStore;
use wasm_bindgen_futures::spawn_local;
use yewdux_functional::use_store;
use yew_router::history::History;
use yew_router::hooks::use_history;
use crate::router::Route;
use crate::store::Store;
use yewdux::dispatch::Dispatcher;

#[derive(Default)]
pub struct State {
    pub username: String,
    pub password: String,
}

#[styled_component(Login)]
pub fn login()->Html {
    let sheetstyle = css!(r#""#);

    let history = use_history().unwrap();
    let store = use_store::<PersistentStore<Store>>();
    let store_dispatch = store.dispatch();

    let onsubmit = {
        let store_dispatch = store_dispatch.clone();
        Callback::from(move |user: User| {
            let history = history.clone();
            let store_dispatch = store_dispatch.clone();
            // log!(&*username_state,&*password_state);
            spawn_local(async move {
                let request = login(user.username, user.password).await;
                history.push(Route::Home);
                let username = request.data.username.clone();
                let token = request.data.token.clone();
                /////// store username and token 
                store_dispatch.reduce(move |state|{
                    state.username = username;
                    state.token = token
                });
            })
        })
    };

    html!{
        <AccountForm onsubmit = {onsubmit} action={Action::Login}/>
    }
}