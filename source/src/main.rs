#![recursion_limit="40"]
#[macro_use]
extern crate diesel;

mod world_server;
mod models;
mod schema;
mod auth_server;
mod utils;
mod master_server;

use base_server::load_config;

use std::thread;

use auth_server::AuthServer;
use world_server::WorldServer;
use crate::master_server::MasterServer;
use diesel::{SqliteConnection, Connection};

/// Runs the server.
fn main() {
	let master_thread = thread::spawn(move || {
		let master: MasterServer = MasterServer {
			conn: SqliteConnection::establish(load_config().db.path[..].into()).unwrap()
		};
		master.start();
	});

	let auth_thread = thread::spawn(move || {
		let auth: AuthServer = AuthServer {};
		auth.start(load_config());
	});

	let world: WorldServer = WorldServer {};
	world.start(load_config(), 2000, 0);

}
