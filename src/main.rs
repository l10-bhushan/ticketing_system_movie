// Movie Ticketing System
mod models;

use models::movies::MovieStruct as Movie;
use models::movies::MoviesDb;
use models::theater::TheaterStruct as Theater;
use models::user::Users;

fn main() {
    println!("Practice rust - Part 2");

    let mut movie_db = MoviesDb::new();
    Movie::set_movie_details(
        &mut movie_db,
        String::from("HomeBound"),
        String::from("Bhushan"),
        8.5,
        String::from("20/11/2025"),
        String::from("7:00 PM IST"),
        300.50,
        100,
        50,
    );

    Movie::set_movie_details(
        &mut movie_db,
        String::from("Kantara - The first chapter"),
        String::from("Bhushan"),
        9.5,
        String::from("20/09/2025"),
        String::from("8:00 PM IST"),
        500.50,
        300,
        10,
    );

    dbg!(movie_db.get_movies_list());
}
