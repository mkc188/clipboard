use iron::prelude::*;
use iron::status;

use views;
use database;
use models::tunnel::Tunnel;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    use diesel::prelude::*;
    use models::schema::tunnels::dsl::*;

    let results = tunnels.filter(created_time.eq(0)).limit(5).load::<Tunnel>(&*database::connection().get().unwrap()).expect("Error loading tunnels");

    let resp = Response::with((status::Ok, views::shared::root().unwrap()));
    Ok(resp)
}
