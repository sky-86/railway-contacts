#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod app;
mod cors;
mod crud;
mod models;
mod schema;

use crate::crud::shared::Db;
use cors::CORS;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(CORS)
        .attach(crud::read::stage())
        .attach(crud::create::stage())
        .attach(crud::delete::stage())
        .attach(crud::update::stage())
        .attach(app::stage())
}
