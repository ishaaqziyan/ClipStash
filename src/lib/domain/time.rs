use chrono::{DateTime, NaiveDataTime,Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::std::FromStr;

#[derive(Clone,Debug,From,Deserialize,Serialize)]
pub struct Time(DateTime<Utc>);


impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
          self.0
    }

    pub fn timestamp(&self)-> i64 {
        self.0.timestamp()
    }

    pub fn from_naive_utc (datetime: NaiveDataTime) -> Self {
        Time(DateTime::from_utc(datetime,Utc))
    }
}

impl FromStr for Time {
    type Err= chrono::ParseError;
    fn from_str(s: &str) -> Result<self, Self::Err> {
        //2023-08-10
        match format!("{}T00:00:00Z",s).parse::<DateTime<Utc>>(){
            Ok(time)=> Ok(time.into()),
            Err(e)=> Err(e),
        }
    }
    
}