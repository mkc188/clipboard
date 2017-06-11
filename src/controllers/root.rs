use iron::prelude::*;
use iron::status;

use views;
use database;
use models::tunnel::Tunnel;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    use diesel::prelude::*;
    use models::schema::tunnels::dsl::*;

    // TODO: move to model
    let results = tunnels.filter(created_time.eq(0)).limit(5).get_result::<Tunnel>(&*database::connection().get().unwrap());
    println!("Displaying {} tunnels", 0);
    for tunnel in results {
        println!("{}", tunnel.mobile_id);
        println!("-----------\n");
        println!("{}", tunnel.computer_id);
    }

    let resp = Response::with((status::Ok, views::shared::root().unwrap()));
    Ok(resp)
}
