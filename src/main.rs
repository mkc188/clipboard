#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate router;
extern crate clipboard;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate maud;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

mod db;
pub mod schema;
pub mod models;

use clipboard::*;
use self::models::*;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io;
use db::Db;

fn main() {
    dotenv().ok();
    
    let db = Db::new();
    let db_connection_pool = db.get_pool();
    
    use self::schema::tunnels::dsl::*;

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let results = tunnels.filter(created_time.eq(0))
            .limit(5)
            .load::<Tunnel>(&connection)
            .expect("Error loading tunnels");
    
        let name = "Lyra";
        let markup = html! {
            p { "Hi, " (name) "!" }
        };

        Ok(Response::with((status::Ok, markup)))
    }

    let _server = Iron::new(hello_world).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}