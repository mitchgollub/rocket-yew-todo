#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

use rocket_contrib::serve::StaticFiles;
fn main() {
    rocket::ignite()
        .mount("/api", routes![hello])
        .mount("/", StaticFiles::from("../../dist"))
        .launch();
}
