#![feature(plugin)]
#![plugin(maud_macros)]

#![allow(dead_code)]


extern crate iron;
extern crate router;
extern crate mount;
extern crate maud;
extern crate params;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
#[macro_use] extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate staticfile;

use iron::prelude::*;
use router::Router;
use mount::Mount;
use dotenv::dotenv;

mod database;
mod controllers;
mod models;

fn main() {
    dotenv().ok();
    let mut index_router = Router::new();
    index_router.get("/", controllers::root::handler, "index");

    let mut mount = Mount::new();
    mount.mount("/", index_router);

    let log_chain = Chain::new(mount);

    Iron::new(log_chain).http("0.0.0.0:3000").unwrap();
}

