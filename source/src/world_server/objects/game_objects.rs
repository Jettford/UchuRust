use crate::utils::log;
use crate::world_server::replica::{ConstructObject};
use crate::common_vars::WorldContext;

use lu_packets::common::{LuWStr, LuVarWString};

pub struct GameObject {
    pub object_id: i64,
    pub lot: i32,

    pub name: String,
}

impl GameObject {
    pub fn construct(&self, mut ctx: &mut WorldContext) {
        let mut vector: Vec<u16> = self.name.encode_utf16().collect();

        ctx.send(ConstructObject {
            network_id: (0 as u16),
            is_construction: true,
            object_id: self.object_id,
            lot: self.lot,
            name_length: vector.len() as u8,
            name: vector,
            time_since_created_on_server: 0,
            has_trigger: false,
            trigger_object_id: 0,
            spawner_node_id: 0,
            object_scale: 0.0,
            object_world_state: 0,
        }).unwrap();
    }
}