#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(dotenv_macros)]

extern crate dotenv;
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv::dotenv().ok();
    println!("{}", &dotenv!("TOKEN"));
    rocket::ignite().mount("/", routes![index]).launch();
}
