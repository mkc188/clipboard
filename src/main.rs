#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate clipboard;
extern crate diesel;
extern crate maud;

use clipboard::*;
use self::models::*;
use diesel::prelude::*;

use iron::prelude::*;
use iron::status;
use std::io;

fn main() {
    use self::schema::tunnels::dsl::*;

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let connection = establish_connection();
        let results = tunnels.filter(created_time.eq(0))
            .limit(5)
            .load::<Tunnel>(&connection)
            .expect("Error loading tunnels");
    
        println!("Displaying {} tunnels", results.len());
        for tunnel in results {
            println!("{}", tunnel.mobile_id);
            println!("-----------\n");
            println!("{}", tunnel.computer_id);
        }

        let name = "Lyra";
        let markup = html! {
            p { "Hi, " (name) "!" }
        };
        // println!("{}", markup.into_string());

        Ok(Response::with((status::Ok, markup.into_string())))
    }

    let _server = Iron::new(hello_world).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}