use std::ops::Deref;

use crate::components::atoms::bb_button::BBButton;
use crate::components::atoms::bb_input_text::InputText;
use crate::components::atoms::bb_input_text::InputType;
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<User>,
    pub action:Action,
}

#[derive(Default,Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(PartialEq,Clone)]
pub enum Action {
    CreateAccount,
    Login,
}
impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::CreateAccount => "Create Account".to_owned(),
            Action::Login => "Login".to_owned(),
        }
    }
}

#[styled_component(AccountForm)]
pub fn account_form(props: &Props) -> Html {
    let sheetstyle = css!(r#""#);

    let state = use_state(User::default);

    let onsubmit = {
        let submit_props = props.onsubmit.clone();
        let state = state.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let user = state.deref().clone();
            submit_props.emit(user);
        })
    };

    let username_change = {
        let user_state = state.clone();
        Callback::from(move |usename: String| {
            log!(usename.clone());
            let mut user = user_state.deref().clone();
            user.username = usename;
            user_state.set(user);
        })
    };
    let password_change = {
        let user_state = state.clone();
        Callback::from(move |password: String| {
            log!(password.clone());
            let mut user = user_state.deref().clone();
            user.password = password;
            user_state.set(user);
        })
    };

    html! {
        <form {onsubmit}>
            <InputText inputtype={InputType::Normal} placeholder={"what's your name?".to_owned()} label={"user name".to_owned()} data_test={"UserName".to_owned()} onchange={username_change} />
            <InputText inputtype={InputType::Password} placeholder={"what's your password?".to_owned()} label={"password".to_owned()} data_test={"Password".to_owned()}  onchange={password_change}/>
            <BBButton label = {props.action.to_string()} data_test = "submit" />
        </form>
    }
}
