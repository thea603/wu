pub use bson;
pub use super::User;
pub use serde_json;


impl User {
    pub fn all() -> String {
        let person = User {
            id: "12345".to_string(),
            username: "Emma".to_string(),
            password: "awef".to_string(),
            own: [1,2,3,4,5,6,7]
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
}
pub fn hello() -> String {
    "こんにちは".to_string()
}
