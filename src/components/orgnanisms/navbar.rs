use yew::prelude::*;
use stylist::{css,yew::styled_component};
use yewdux::prelude::PersistentStore;
use crate::components::atoms::bb_link::NavBarLink;
use crate::router::Route;
use yewdux_functional::use_store;
use crate::store::Store;

#[styled_component(NavBar)]
pub fn navbar()->Html{
    let sheetstyle = css!{
        r#"
            border-bottom : 1px solid antiquewhite;
            padding : 10px 20px;
        "#
    };
    let store = use_store::<PersistentStore<Store>>();

    let username = store.state().map(|state|{
        state.username.clone()
    }).unwrap_or_default();

    let token = store.state().map(|state|{
        state.token.clone()
    }).unwrap_or_default();

    html!{
        <selection class={sheetstyle}>
         <NavBarLink text={"ToDo".to_owned()}  data_test={"".to_owned()} router={Route::Home} />
         if token.is_empty() {
            <NavBarLink text={"Create Account".to_owned()}  data_test={"create-account".to_owned()} router={Route::CreateAccount} />
         }else{
            <div>{&username}</div>
         }
         <NavBarLink text={"Login".to_owned()}  data_test={"login".to_owned()} router={Route::Login} />
        </selection>
    }
}