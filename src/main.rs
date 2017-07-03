#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate wuligege;
extern crate json;

use wuligege::user::User;
use rocket::response::content;


#[get("/")]
fn index() -> content::JSON<String>  {
    let user = User::new("simon".to_string(),"123666".to_string());
    content::JSON(user.to_json())
}

#[get("/")]
fn hello() -> String {
    let mut x = 5;
    let y = &mut x;

    *y += 1;
    // "hello".to_string()
    // y.to_string()
    let  goodbye = wuligege::japanese::farewells::goodbye();
    let  greetings =  wuligege::japanese::greetings::hello();
    let helloworld = goodbye +"这是牛逼的"+ &greetings;
    helloworld
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "Hello, world!"
}





fn main() {

    rocket::ignite()
    .mount("/", routes![index])
    .mount("/hello", routes![hello])
    .mount("/hello", routes![world])
    .launch();

}
