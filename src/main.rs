extern crate iron;
extern crate clipboard;
extern crate diesel;

use clipboard::*;
use self::models::*;
use diesel::prelude::*;

use iron::prelude::*;
use iron::status;

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
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let _server = Iron::new(hello_world).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}