use crate::models::movies::MovieStruct;

pub struct TheaterStruct {
    name: String,
    location: String,
    seat_available: bool,
    movies: MovieStruct,
}
