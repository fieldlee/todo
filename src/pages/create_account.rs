use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux_functional::use_store;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::PersistentStore;
use crate::router::Route;
use crate::store::Store;
use crate::api::createAccount;
use yewdux::dispatch::Dispatcher;
use crate::components::molecules::account_form::{AccountForm,User,Action};



#[derive(Default)]
pub struct State {
    pub username: String,
    pub password: String,
}


#[function_component(CreateAccount)]
pub fn create_account() -> Html {
    // let state = use_state(State::default);
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
                let request = createAccount(user.username, user.password).await;
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

    html! {
        <AccountForm onsubmit = {onsubmit} action = {Action::CreateAccount}/>
    }

}
