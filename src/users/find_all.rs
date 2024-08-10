use super::user::User;
use diesel::prelude::*;

pub fn find_all() -> Vec<User> {
    use crate::schema::users::dsl::*;
    use crate::establish_connection;

    let connection = &mut establish_connection();

    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    results

    // println!("Displaying {} users", results.len());


    // for user in &results {
    //     println!("{}", user.id);
    //     println!("-----------\n");
    //     println!("{}", user.email);
    //     println!("-----------\n");
    //     println!("{}", user.created_at);
    // }

    // println!("Displaying {} users", results.len());
}