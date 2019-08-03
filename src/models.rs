pub mod pagination;

use crate::schema::*;
use diesel::sqlite::{SqliteConnection};
use diesel::prelude::*;
use diesel::query_builder::*;
use juniper_eager_loading::impl_load_from_for_diesel;

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub country_id: i32,
}

#[derive(Queryable, Debug, Clone)]
pub struct Country {
    pub id: i32,
    pub name: String,
}

// impl_load_from_for_diesel! {
//     (
//         error = diesel::result::Error,
//         connection = SqliteConnection,
//     ) => {
//         i32 -> (users, User),
//         i32 -> (countries, Country),
//     }
// }
//
// Manually edited ouput of this macro.


impl juniper_eager_loading::LoadFrom<i32> for User {
    type Error = diesel::result::Error;
    type Connection = SqliteConnection;
    fn load(ids: &[i32], db: &Self::Connection) -> Result<Vec<Self>, Self::Error> {
        users::table
            .filter(users::id.eq_any(ids))
            .load::<User>(db)
            .map_err(From::from)
    }
}
impl juniper_eager_loading::LoadFrom<i32> for Country {
    type Error = diesel::result::Error;
    type Connection = SqliteConnection;
    fn load(ids: &[i32], db: &Self::Connection) -> Result<Vec<Self>, Self::Error> {
        countries::table
            .filter(countries::id.eq_any(ids))
            .load::<Country>(db)
            .map_err(From::from)
    }
}