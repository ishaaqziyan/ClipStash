use serde::{Deserialize, Serialize};
use derive_more::Constructor;
use crate::domain::Time;

#[derive(Clone,Constructor,Debug,Deserialize,Serialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0    
    }
}