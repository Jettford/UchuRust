pub(crate) mod server_mode;

use actix_web::{get, web, App, HttpServer, Responder};

use server_mode::ServerMode;

use crate::utils::log;
use diesel::{QueryDsl, TextExpressionMethods, SqliteConnection, Connection};

pub struct MasterServer {
    pub(crate) conn: SqliteConnection,
}

impl MasterServer {
    #[actix_web::main]
    pub async fn start(&self) -> std::io::Result<()> {
        log("Master", "Started master server");

        #[get("/verify/{username}/{session_key}")]
        async fn main_path(web::Path((username, session_key)): web::Path<(String, String)>) -> impl Responder {
            check_session_key(username, session_key)
        }

        HttpServer::new(|| App::new().service(main_path)).bind("127.0.0.1:21835")?.run().await
    }


    fn check_session_key(&self, incoming_username: String, incoming_session_key: String) -> String {
        use crate::schema::users::dsl::{users, username, session_key};
        use crate::models::User;

        let user: User = match users.filter(username.like(incoming_username)).first::<User>(&self.conn) {
            Err(err) => {
                return String::from("0");
            }
            Ok(x) => x,
        };

        if user.session_key == incoming_session_key { return String::from("1"); }

        return String::from("0");
    }

}

