#![allow(warnings)]
// Movie Ticketing System
use std::io;
use std::io::Write;
use std::process;
mod models;
mod utils;

use models::movies::MovieStruct as Movie;
use models::movies::MoviesDb;
use models::theater::TheaterStruct as Theater;
use models::user::User;

use crate::models::user::UsersDb;
use crate::utils::helper;

fn main() {
    let mut user_db: UsersDb;
    println!("Welcome to movie cli\n");

    println!("X: New User");
    println!("Y: Login");

    print!("\nPlease select from above the two options X/Y : ");
    io::stdout().flush().expect("");
    let mut user_input = String::from("");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error : Failed to read value");

    let user_input: char = user_input.trim().parse().expect("Error: Not able to parse");
    println!("You choose : {}", user_input);

    match user_input {
        'X' | 'x' => {
            user_db = helper::initialize_db();
            User::new(&mut user_db);
        }
        'Y' | 'y' => println!("User logged in boom"),
        _ => {
            println!("Please choose a valid option");
            process::exit(1);
        }
    }

    let mut movie_db = MoviesDb::new();
    Movie::initialze(&mut MoviesDb::new());

    dbg!(movie_db.get_movies_list());
}
