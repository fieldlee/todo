use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;
use crate::router::Route;

#[derive(Properties,Clone,PartialEq)]
pub struct Props {
    pub text:String,
    pub data_test:String,
    pub router: Route,
}

#[styled_component(NavBarLink)]
pub fn navbarlink(props : &Props) -> Html{
    let sheetstyle = css!(r#"

    "#);
    html!{
        <span data_test={props.data_test.clone()}>
            <Link<Route> to={props.router.clone()} classes={classes!(sheetstyle.clone())}> {props.text.clone()} </Link<Route>>
        </span>
        
    }
}