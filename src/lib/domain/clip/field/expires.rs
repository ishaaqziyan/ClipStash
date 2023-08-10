use serde::{Deserialize, Serialize};
use std::std::FromStr;
use super::ClipEror; 
use crate::domain::Time;

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<t: Into<Option<Time>>>(expires:T) -> Self {
        Self(expires.into)
    }
    pub fn into_inner(self) ->Option<Time> {
       self    }
}

impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err= ClipError;
    fn from_str(raw: &str) ->Result<Self, Self::Err> {
        if raw.is_empty() {
            Ok(Self(None))
        } else {
            match Time::from_str(raw){
                Ok(time) => Ok(Self::new(time)),
                Err(e)=> Err(e.into())
            }
        }
    }
}