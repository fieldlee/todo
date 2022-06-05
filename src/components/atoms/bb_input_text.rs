use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum InputType {
    Normal,
    Password,
}

impl ToString for InputType {
    fn to_string(&self) -> String {
        match self {
            InputType::Normal => "text".to_owned(),
            InputType::Password => "password".to_owned(),
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub data_test: String,
    pub placeholder: Option<String>,
    pub inputtype: InputType,
    pub onchange: Callback<String>,
}

#[styled_component(InputText)]
pub fn input_text(props: &Props) -> Html {
    let sheetstyle = css!(
        r#"

    "#
    );

    let onchange = {
        let emit_change = props.onchange.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
                emit_change.emit(value);
        })
    };

    let id = props
        .label
        .clone()
        .to_lowercase()
        .replace(" ", "-")
        .to_owned();

    html! {
        <>
            <label for={id.clone()}>{props.label.clone()}</label>
            <input type={props.inputtype.to_string()} id = {id.clone()} placeholder ={props.placeholder.clone().unwrap_or_default()}  data_test={props.data_test.clone()} {onchange}/>
        </>
    }
}
