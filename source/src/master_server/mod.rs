pub(crate) mod server_mode;

#[macro_use]

use rouille::*;

use crate::utils::log;
use diesel::{QueryDsl, TextExpressionMethods, SqliteConnection, Connection};
use diesel::sqlite::Sqlite;

pub struct MasterServer {

}

impl MasterServer {
    pub fn start(&self) -> std::io::Result<()> {
        log("Master", "Started master server");

        use base_server::load_config;

        rouille::start_server("localhost:21835",  |request| {
            router!(request,
            (GET) (/) => {
                rouille::Response::empty_404()
            },
            (GET) (/verify/{incoming_username: String}/{incoming_session_key: String}) => {
                rouille::Response::text(check_session_key(&SqliteConnection::establish(load_config().db.path[..].into()).unwrap(), incoming_username, incoming_session_key))
            },
            _ => rouille::Response::empty_404()
            )
        });
    }
}

fn check_session_key(conn: &SqliteConnection, incoming_username: String, incoming_session_key: String) -> String {
    use crate::schema::users::dsl::{users, username};
    use crate::models::User;
    use crate::diesel::RunQueryDsl;

    let mut result: String = String::from("0");

    let user: User = match users.filter(username.like(String::from(incoming_username))).first::<User>(conn) {
        Err(_) => {
            return result;
        }
        Ok(x) => x,
    };

    if user.session_key == incoming_session_key { result = String::from("1") }

    return result;
}