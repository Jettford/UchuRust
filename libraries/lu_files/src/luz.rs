use crate::internal_classes::*;
use std::collections::HashMap;
use std::fs::File;

pub struct Scene {
    pub luz_version: u32,
    pub world_id: u32,
    pub file_name: String,
    pub scene_id: u32,
    pub layer_id: u32,
    pub scene_name: String,
}

pub struct LUZone {
    pub version: u32,
    pub version_control: u32,
    pub world_id: u32,

    pub spawn: Vector3,
    pub spawn_rot: Quaternion,

    pub scene_count: u32,
    pub scenes: Vec<Scene>,
}

impl LUZone {
    fn read(&mut self, path: String) {
        let mut file = File::open(path).unwrap();

        self.version = file.read_u32().unwrap();

        if self.version >= 0x24 {
            self.version_control = file.read_u32().unwrap();
        }

        self.world_id = file.read_u32().unwrap();

        if self.version >= 0x26 {
            self.spawn.x = file.read_f32().unwrap();
            self.spawn.y = file.read_f32().unwrap();
            self.spawn.z = file.read_f32().unwrap();

            self.spawn_rot.x = file.read_f32().unwrap();
            self.spawn_rot.y = file.read_f32().unwrap();
            self.spawn_rot.z = file.read_f32().unwrap();
            self.spawn_rot.w = file.read_f32().unwrap();
        }

        if self.version < 0x25 {
            self.scene_count = file.read_u8().unwrap() as u32;
        } else {
            self.scene_count = file.read_u32().unwrap();
        }

        for i in 0..self.scene_count {
            let mut file_name: Vec<u8> = vec![];

            for i in 0..file.read_u8().unwrap() {
                file_name.push(file.read_u8().unwrap());
            }

            let scene_id = file.read_u32().unwrap();
            let layer_id = file.read_u32().unwrap();

            let mut scene_name: Vec<u8> = vec![];

            for i in 0..file.read_u8().unwrap() {
                scene_name.push(file.read_u8().unwrap());
            }

            file.read_u24();

            Scene {
                luz_version: self.version,
                world_id: self.world_id,
                file_name: String::from(file_name),
                scene_id,
                layer_id,
                scene_name: String::from(scene_name)
            }
        }
    }
}