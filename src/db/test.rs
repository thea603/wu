pub use bson::Bson;
pub use mongodb::{Client, ThreadedClient};
pub use mongodb::db::ThreadedDatabase;
pub fn testone() -> String {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("test").collection("movies");

    let doc = doc! { "title" => "Jaws",
                      "array" => [ 1, 2, 3 ] };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    // cursor.next() returns an Option<Result<Document>>
    let mut ptitle = "".to_string();
    match item {
        Some(Ok(doc)) => match doc.get("_id") {
            Some(&Bson::ObjectId(ref _id)) => ptitle = ptitle+&(_id.to_hex()),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
    ptitle
}
