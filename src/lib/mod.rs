pub mod data;
pub mod domain;
pub mod service;
pub mod web;


use data::AppDatabase;
pub use domain::clip::field::ShortCode;
pub use domain::clip::{Clip,ClipError};
pub use domain::time::Time;
pub use data::DataError;
pub use service::ServiceError;



use rocket::fs::FileServer;
use rocket::{Build,Rocket};
use web::hitcounter::HitCounter;
use web::{renderer::Renderer};


pub fn rocket(config: RocketConfig) -> Rocket<Build>{
    rocket::build()
       .manage::<AppDatabase>(config.database)
       .manage::<Renderer>(config.renderer)
       .manage::<HitCounter>(config.hit_counter)
       .mount("/", web::http::routes())
       .mount("/api/clip", web::api::routes())
       .mount("/static", FileServer::from("static"))
       .register("/", web::http::catcher::catchers())
       .register("/api/clip", web::api::catcher::catchers())
}

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
    pub hit_counter: HitCounter,
}
