use iron::prelude::*;
use iron::status;

use views;
use models::tunnel::Tunnel;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, views::shared::root().unwrap()));
    Ok(resp)
}
