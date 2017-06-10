extern crate clipboard;
extern crate diesel;

use clipboard::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::tunnels::dsl::*;

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
}
