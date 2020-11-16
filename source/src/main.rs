#![recursion_limit="40"]
#[macro_use]
extern crate diesel;

mod world_server;
mod auth_server;
mod utils;
mod master_server;
mod internal_structs;
mod common_vars;
mod database;

use base_server::load_config;

use std::thread;

use auth_server::AuthServer;
use world_server::WorldServer;
use crate::master_server::MasterServer;
use diesel::{SqliteConnection, Connection};

/// Runs the server.
fn main() {
	let master_thread = thread::spawn(move || {
		let master: MasterServer = MasterServer {};
		master.start();
	});

	let auth_thread = thread::spawn(move || {
		let auth: AuthServer = AuthServer {};
		auth.start(load_config());
	});

	let world: WorldServer = WorldServer {};
	world.start(load_config(), 2000, 0);

}
