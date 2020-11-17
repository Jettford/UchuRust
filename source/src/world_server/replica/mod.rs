use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize, EWrite};
use endio::LittleEndian as LE;
use byteorder::{WriteBytesExt, LittleEndian};

use lu_packets::common::LuWStr;
use endio_bit::{LEBitWriter, BitWriter};
use std::fs::File;

pub struct ConstructObject {
    pub network_id: u16,
    pub is_construction: bool,
    pub object_id: i64,
    pub lot: i32,
    pub name_length: u8,
    pub name: Vec<u16>,
    pub time_since_created_on_server: u32,
    pub has_trigger: bool,
    pub trigger_object_id: i64,
    pub spawner_node_id: u32,
    pub object_scale: f32,
    pub object_world_state: u8,
}

impl ConstructObject {
    pub fn serialize(self) -> Vec<u8> {
        let mut bit_writer = ::endio_bit::LEBitWriter::new(vec![]);

        bit_writer.write_bit(true).unwrap();
        bit_writer.write_u16::<LittleEndian>(self.network_id).unwrap();

        if self.is_construction {
            bit_writer.write_i64::<LittleEndian>(self.object_id).unwrap();
            bit_writer.write_i32::<LittleEndian>(self.lot).unwrap();
            bit_writer.write_u8(self.name_length).unwrap();

            for i in 0..self.name_length {
                bit_writer.write_u16::<LittleEndian>(self.name[i as usize]).unwrap();
            }

            bit_writer.write_u32::<LittleEndian>(self.time_since_created_on_server).unwrap();

            bit_writer.write_bit(false).unwrap();
            bit_writer.write_bit(false).unwrap();

            bit_writer.write_bit(self.has_trigger).unwrap();
            bit_writer.write_bit(self.has_trigger).unwrap();
            if self.has_trigger {
                bit_writer.write_i64::<LittleEndian>(self.trigger_object_id).unwrap();
            }

            bit_writer.write_bit( self.spawner_node_id != 0).unwrap();
            if self.spawner_node_id != 0 {
                bit_writer.write_u32::<LittleEndian>( self.spawner_node_id).unwrap();
            }
            bit_writer.write_bit(self.object_scale != 0.0).unwrap();
            if self.object_scale != 0.0 {
                bit_writer.write_f32::<LittleEndian>(self.object_scale).unwrap();
            }
            bit_writer.write_bit(self.object_world_state != 0).unwrap();
            if self.object_world_state != 0 {
                bit_writer.write_u8(self.object_world_state).unwrap();
            }
            bit_writer.write_bit(false).unwrap();
        }
        bit_writer.write_bit(true).unwrap();
        bit_writer.write_bit(false).unwrap();
        bit_writer.write_bit(false).unwrap();

        //bit_writer.align();

        let data: &Vec<u8> = bit_writer.get_mut();

        data.clone()
    }
}