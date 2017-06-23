
pub mod english;
pub mod japanese;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
pub mod db;
extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use] extern crate serde_derive;
pub mod user;
