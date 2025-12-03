use crate::models::movies::MovieStruct;

pub struct TheaterDb {
    theaters: Vec<TheaterStruct>,
}
pub struct TheaterStruct {
    name: String,
    location: String,
    seat_available: bool,
    movies: Vec<MovieStruct>,
}

// impl TheaterStruct {
//     fn initialze() -> Self {
//         Self {
//             name: (),
//             location: (),
//             seat_available: (),
//             movies: (),
//         }
//     }
// }
