use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties,Clone,PartialEq)]
pub struct Props {
    pub label:String,
    pub data_test:String,
}

#[styled_component(BBButton)]
pub fn bb_button(props : &Props) -> Html{
    let sheetstyle = css!(r#"

    "#);
    html!{
        <button data-test={props.data_test.clone()} >{props.label.clone()}</button>
    }
}