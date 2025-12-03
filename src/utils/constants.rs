use crate::models::movies::MovieStruct;
use uuid::Uuid;

pub fn json_to_array() -> Vec<MovieStruct> {
    vec![
        MovieStruct {
            movie_id: Uuid::new_v4(),
            name: String::from("HomeBound"),
            director: String::from("Bhushan"),
            rating: 8.5,
            launch_date: String::from("20/11/2025"),
            time: String::from("7:00 PM IST"),
            ticket_price: 300.50,
            no_of_tickets_sold: 100,
            no_of_tickets_unsold: 50,
        },
        MovieStruct {
            movie_id: Uuid::new_v4(),
            name: String::from("Kantara - The first chapter"),
            director: String::from("Bhushan"),
            rating: 9.5,
            launch_date: String::from("20/09/2025"),
            time: String::from("8:00 PM IST"),
            ticket_price: 500.50,
            no_of_tickets_sold: 300,
            no_of_tickets_unsold: 10,
        },
    ]
}
