mod auth_listener;

use base_server::server::Server;
use base_server::{Config, create_tls_config};

use crate::utils;

use lu_packets::{
    auth::client::Message as OutMessage,
    auth::server::Message as IncMessage,
};

use crate::auth_server::auth_listener::AuthMsgCallback;

pub struct AuthServer {

}

impl AuthServer {
    pub fn start(&self, config: Config) {
        let tls_config = create_tls_config(config.tls);
        let listener = AuthMsgCallback::new(config.db.path[..].into());
        let mut server = Server::<IncMessage, OutMessage, _>::new("0.0.0.0:21836", tls_config, |i, o| AuthMsgCallback::on_msg(&listener, i, o)).unwrap();
        utils::log("Auth", "Started");
        server.run();
    }
}