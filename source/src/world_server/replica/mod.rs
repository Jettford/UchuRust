use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize, EWrite};
use endio::LittleEndian as LE;
use byteorder::{WriteBytesExt, LittleEndian};

use lu_packets::common::LuWStr;
use endio_bit::{LEBitWriter, BitWriter};

#[derive(Debug, PartialEq)]
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

impl<'a> Serialize<LE, Vec<u8>> for &'a ConstructObject {
    fn serialize(self, writer: &mut W) -> Res<()> {
        let mut bit_writer = ::endio_bit::LEBitWriter::new(vec![]);

        bit_writer.write_bit(true)?;
        bit_writer.write_u16::<LittleEndian>(self.network_id)?;

        if self.is_construction {
            bit_writer.write_i64::<LittleEndian>(self.object_id)?;
            bit_writer.write_i32::<LittleEndian>(self.lot)?;
            bit_writer.write_u8(self.name_length)?;

            for i in 0..self.name_length {
                bit_writer.write_u16::<LittleEndian>(self.name[i as usize])?;
            }

            bit_writer.write_u32::<LittleEndian>(self.time_since_created_on_server)?;

            bit_writer.write_bit(false)?;
            bit_writer.write_bit(false)?;

            bit_writer.write_bit(self.has_trigger)?;
            bit_writer.write_bit(self.has_trigger)?;
            if self.has_trigger {
                bit_writer.write_i64::<LittleEndian>(self.trigger_object_id)?;
            }

            bit_writer.write_bit( self.spawner_node_id != 0)?;
            if self.spawner_node_id != 0 {
                bit_writer.write_u32::<LittleEndian>( self.spawner_node_id)?;
            }
            bit_writer.write_bit(self.object_scale != 0.0)?;
            if self.object_scale != 0.0 {
                bit_writer.write_f32::<LittleEndian>(self.object_scale)?;
            }
            bit_writer.write_bit(self.object_world_state != 0)?;
            if self.object_world_state != 0 {
                bit_writer.write_u8(self.object_world_state)?;
            }
            bit_writer.write_bit(false)?;
        }
        bit_writer.write_bit(true)?;
        bit_writer.write_bit(false)?;
        bit_writer.write_bit(false)?;

        let data: &Vec<u8> = bit_writer.get_mut();
        for i in 0..data.len() {
            writer.write_u8(data[i])?;
        }

        Ok(())
    }
}