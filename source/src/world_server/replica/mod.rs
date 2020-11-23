mod components;

use std::fs::File;
use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize, EWrite, BigEndian};

use endio_bit::{BEBitWriter, BitWriter};

use byteorder::{WriteBytesExt, LittleEndian};

use lu_packets::common::LuWStr;

pub struct ConstructObject {
    pub network_id: u16,
    pub is_construction: bool,
    pub object_id: i64,
    pub lot: i32,
    pub name_length: u8,
    pub name: Vec<u16>,
    pub time_since_created_on_server: u32,
}

impl ConstructObject {
    pub fn serialize(self) -> Vec<u8> {
        let mut bit_writer = ::endio_bit::BEBitWriter::new(vec![]);

        bit_writer.write_u8(0x24).unwrap();

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

            bit_writer.write_bit(false).unwrap(); // Compressed LDF
            bit_writer.write_bit(false).unwrap(); // Trigger exist?

            bit_writer.write_bit( false).unwrap(); // spawner?
            bit_writer.write_bit( false).unwrap(); // spawner node?

            bit_writer.write_bit(true).unwrap();
            bit_writer.write_f32::<LittleEndian>(1.0).unwrap();

            bit_writer.write_bit(false).unwrap(); // object world state
            bit_writer.write_bit(false).unwrap(); // gmlevel
        }
        bit_writer.write_bit(true).unwrap(); // child and parent stuff, ignore for now
        bit_writer.write_bit(false).unwrap();
        bit_writer.write_bit(false).unwrap();

        let data: &Vec<u8> = bit_writer.get_ref();

        data.clone()
    }
}

pub trait Component {
    fn get_id(&self) -> u32;

    //fn serialize(&self, writer: BitWriter<BigEndian, Vec<u8>>) { }




}