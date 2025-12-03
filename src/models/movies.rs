use crate::utils::{constants, helper};
use uuid::Uuid;

pub struct MoviesDb {
    movies_list: Vec<MovieStruct>,
}
#[derive(Debug)]
pub struct MovieStruct {
    pub movie_id: Uuid,
    pub name: String,
    pub director: String,
    pub rating: f64,
    pub launch_date: String,
    pub time: String,
    pub ticket_price: f64,
    pub no_of_tickets_sold: u16,
    pub no_of_tickets_unsold: u16,
}

impl MoviesDb {
    pub fn new() -> Self {
        Self {
            movies_list: vec![],
        }
    }

    pub fn get_movies_list(&self) -> &Vec<MovieStruct> {
        &self.movies_list
    }
}

impl MovieStruct {
    pub fn initialze(movie_db: &mut MoviesDb) {
        let movies_list = constants::json_to_array();
        for item in &movies_list {
            let movie = helper::set_movie_details(item);
            movie_db.movies_list.push(movie);
        }
    }
}
