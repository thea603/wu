pub use bson;
use bson::spec::ElementType::ObjectId;
pub use super::User;
pub use serde_json;


// trait User {
//     fn to_json(&self) -> String;
// }

impl User {
    pub fn all() -> String {
        let person = User {
            id: "12345".to_string(),
            username: "Emma".to_string(),
            password: "awef".to_string(),
        };

        // Convert the Point to a packed JSON string. To convert it to
           // pretty JSON with indentation, use `to_string_pretty` instead.
           let serialized = serde_json::to_string(&person).unwrap();

           // Prints serialized = {"x":1,"y":2}
           println!("serialized = {}", serialized);

           // Convert the JSON string back to a Point.
           let deserialized: User = serde_json::from_str(&serialized).unwrap();

           // Prints deserialized = Point { x: 1, y: 2 }
           println!("deserialized = {:?}", deserialized);
           serialized

    }
    pub fn new(uname:String, upass:String)->User{
        let user = User {
            id: "null".to_string(),
            username: uname,
            password: upass
        };
        user
    }

    pub fn new()-> User(){
        let user = User {
            id: "null".to_string(),
            username: "null".to_string(),
            password: "null".to_string(),
        };
        user
    }

    pub fn save(&self) -> i32{
        -1
    }

    pub fn to_json(&self)-> String{
        let serialized = serde_json::to_string(&self).unwrap();
        serialized
    }

}
