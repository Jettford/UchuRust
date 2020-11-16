use lu_packets::{
    world::client::{Message as OutMessage},
    world::server::{Message as IncMessage},
};

use base_server::server::Context as C;

pub type WorldContext = C<IncMessage, OutMessage>;

type s64 = i64;
type s32 = i32;
type s16 = i16;
type s8 = i8;
