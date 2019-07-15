#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod other {
    #[get("/aruba")]
    pub fn world() -> &'static str {
        "Hello Aruba!"
    }
}

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!"
}
fn main() {
    println!("Hello, world!");

    rocket::ignite()
        .mount("/hello", routes![index, other::world])
        .launch();
}
