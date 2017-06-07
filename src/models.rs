#[derive(Queryable)]
pub struct Tunnel {
    pub id: i32,
    pub mobile_id: String,
    pub computer_id: String,
    pub created_time: i32,
}
