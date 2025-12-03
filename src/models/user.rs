use chrono::{DateTime, Utc};
use std::io::{self, Write, stdout};
use uuid::Uuid;

use crate::utils::helper;
enum UserRole {
    Admin,
    TheaterAdmin,
    Noob,
}

pub struct UsersDb {
    users: Vec<User>,
}

impl UsersDb {
    pub fn new() -> Self {
        Self { users: vec![] }
    }
}

pub struct User {
    user_id: Uuid,
    username: String,
    password: String,
    name: String,
    active: bool,
    joinedAt: DateTime<Utc>,
}

impl User {
    const error: &str = "Error : Reading value";
    pub fn new(usersdb: &mut UsersDb) {
        let (mut username, mut name, mut password, mut active, mut joinedAt): (
            String,
            String,
            String,
            bool,
            DateTime<Utc>,
        ) = (
            String::from(""),
            String::from(""),
            String::from(""),
            false,
            Utc::now(),
        );
        println!("Please enter the details below:\n");

        print!("Name : ");
        stdout().flush().expect("");
        helper::get_user_inp(&mut username);
        helper::get_user_inp(&mut name);
        helper::get_user_inp(&mut password);

        let user = User {
            user_id: Uuid::new_v4(),
            username,
            name,
            password,
            active,
            joinedAt,
        };
    }
}
