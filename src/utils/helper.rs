use crate::models::{movies::MovieStruct, user::UsersDb};
use std::io;

pub fn set_movie_details(item: &MovieStruct) -> MovieStruct {
    MovieStruct {
        movie_id: item.movie_id,
        name: item.name.clone(),
        director: item.director.clone(),
        rating: item.rating,
        launch_date: item.launch_date.clone(),
        time: item.time.clone(),
        ticket_price: item.ticket_price,
        no_of_tickets_sold: item.no_of_tickets_sold,
        no_of_tickets_unsold: item.no_of_tickets_unsold,
    }
}

pub fn get_user_inp(buffer: &mut String) {
    io::stdin().read_line(buffer).expect("Error reading value");
}

pub fn initialize_db() -> UsersDb {
    let user_db = UsersDb::new();
    return user_db;
}
