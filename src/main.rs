#[macro_use]

extern crate diesel;
mod db;
mod models;
mod schema;
mod posts;

use models::*;
use diesel::prelude::*;
use schema::posts::dsl::*;

fn main() {
    println!("Listing posts...");
    let connection = db::establish_connection();
    let new_post = Post { id: 1, title: String::from("Hello world"), body: String::from("This is the body"), published: true };
    let post_result = posts::create_post(&connection, new_post);
    // match post_result {
    //     Ok
    // }



    post_result
        .map(|person| println!("Created"))
        .map_err(|error| println!("{}", error));
}
