use bson;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub own: [i32; 7],
}

pub mod users;
