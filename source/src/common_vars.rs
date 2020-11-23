use lu_packets::{
    world::client::{Message as OutMessage},
    world::server::{Message as IncMessage},
};

use base_server::server::Context as C;

pub type WorldContext = C<IncMessage, OutMessage>;
