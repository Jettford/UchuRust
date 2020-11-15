use std::convert::TryFrom;

//use bcrypt;

use diesel::prelude::*;

use base_server::listeners::{on_conn_req, on_internal_ping, on_handshake};
use base_server::server::Context as C;

use crate::models::User;

use lu_packets::common::{ServiceId, LuWString33, LuString33};
use lu_packets::auth::{
    client::{LoginResponse, Message as OutMessage},
    server::{LoginRequest, Message as IncMessage, LuMessage::Auth},
};

use crate::utils::log;

type Context = C<IncMessage, OutMessage>;

pub struct AuthMsgCallback {
    conn: SqliteConnection,
}

impl AuthMsgCallback {
    pub fn new(db_path: &str) -> Self {
        let conn = SqliteConnection::establish(db_path).unwrap();
        Self { conn }
    }

    pub fn on_msg(&self, msg: &IncMessage, ctx: &mut Context) {
        use lu_packets::raknet::server::Message::{InternalPing, ConnectionRequest, NewIncomingConnection, UserMessage};
        use lu_packets::auth::server::{
            LuMessage::{General},
            GeneralMessage::Handshake,
            AuthMessage::LoginRequest
        };
        match msg {
            InternalPing(msg) => on_internal_ping::<IncMessage, OutMessage>(msg, ctx),
            ConnectionRequest(msg) => on_conn_req::<IncMessage, OutMessage>(msg, ctx),
            NewIncomingConnection(msg) => { log("Auth", "New incoming connection") },
            UserMessage(General(Handshake(msg))) => on_handshake::<IncMessage, OutMessage>(msg, ctx, ServiceId::Auth),
            UserMessage(Auth(LoginRequest(msg))) => self.on_login_request(msg, ctx),
            _ => { log("Auth", "Received unknown packet") },
        }
    }

    pub fn on_login_request(&self, event: &LoginRequest, ctx: &mut Context) {
        use crate::schema::users::dsl::{users, username, session_key};

        let user: User = match users.filter(username.like(String::from(&event.username))).first::<User>(&self.conn) {
            Err(err) => {
                dbg!(err);
                ctx.send(LoginResponse::InvalidUsernamePassword).unwrap();
                return;
            }
            Ok(x) => x,
        };

        //if !bcrypt::verify(String::from(&event.password), &user.password).unwrap() {
        if String::from(&event.password) != user.password {
            ctx.send(LoginResponse::InvalidUsernamePassword).unwrap();
            return;
        };

        let new_session_key: &str = &user.password[..];

        diesel::update(users.find(user.id)).set(session_key.eq(&new_session_key)).execute(&self.conn).unwrap();

        let redirect_address = (LuString33::try_from(String::from("localhost").as_ref()).unwrap(), 2000 as u16);
        let message = LoginResponse::Ok {
            session_key: LuWString33::try_from(new_session_key).unwrap(),
            redirect_address,
        };

        ctx.send(message).unwrap();
    }
}
