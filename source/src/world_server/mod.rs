mod world_listeners;
mod replica;
mod objects;

use base_server::{Config, create_tls_config};
use base_server::server::Server;

use crate::world_server::world_listeners::WorldMsgCallback;

use crate::utils;

use lu_packets::{
    world::client::Message as OutMessage,
    world::server::Message as IncMessage,
};

pub struct WorldServer {

}

impl WorldServer {
    pub fn start(&self, config: Config, port: u16, world_id: u16) {
        let tls_config = create_tls_config(config.tls);
        let mut listener = WorldMsgCallback::new(&config.db.path, String::from("localhost:21835"));
        let mut server = Server::<IncMessage, OutMessage, _>::new(format!("0.0.0.0:{}", port)[..].into(), tls_config, |i, o| WorldMsgCallback::on_msg(&mut listener, i, o)).unwrap();
        utils::log("World", format!("Started world server on port {}, world id: {}", port, world_id)[..].into());
        server.run();
    }
}