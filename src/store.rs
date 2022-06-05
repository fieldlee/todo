use yewdux::prelude::*;
use serde::{Deserialize,Serialize};

#[derive(Debug,Default,Clone,Serialize,Deserialize)]
pub struct Store {
    pub username : String,
    pub token : String,
}

impl Persistent for Store {
    fn key() -> &'static str {
        std::any::type_name::<Self>()
    }
    fn area() -> Area {
        Area::Local
    }
}