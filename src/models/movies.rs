use std::io;
use uuid::Uuid;

pub struct MoviesDb {
    movies_list: Vec<MovieStruct>,
}
#[derive(Debug)]
pub struct MovieStruct {
    movie_id: Uuid,
    name: String,
    director: String,
    rating: f64,
    launch_date: String,
    time: String,
    ticket_price: f64,
    no_of_tickets_sold: u16,
    no_of_tickets_unsold: u16,
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
    pub fn set_movie_details(
        movie_db: &mut MoviesDb,
        name: String,
        director: String,
        rating: f64,
        launch_date: String,
        time: String,
        ticket_price: f64,
        no_of_tickets_sold: u16,
        no_of_tickets_unsold: u16,
    ) {
        let movie = MovieStruct {
            movie_id: Uuid::new_v4(),
            name,
            director,
            rating,
            launch_date,
            time,
            ticket_price,
            no_of_tickets_sold,
            no_of_tickets_unsold,
        };
        movie_db.movies_list.push(movie);
    }
}
