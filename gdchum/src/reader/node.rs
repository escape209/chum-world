use crate::chumfile::ChumFile;
use crate::util;
use gdnative::*;
use libchum::reader::node;

pub fn read_node(data: &Vec<u8>, fmt: libchum::format::TotemFormat) -> Option<Dictionary> {
    let node = match node::Node::read_from(&mut data.as_slice(), fmt) {
        Ok(x) => x,
        Err(e) => {
            godot_print!("NODE file invalid: {}", e);
            return None;
        }
    };
    let mut data = Dictionary::new();
    data.set(&"parent_id".into(), &node.node_parent_id.to_variant());
    data.set(&"resource_id".into(), &node.resource_id.to_variant());
    data.set(&"global_transform".into(), &util::mat4x4_to_transform(&node.global_transform).to_variant());
    data.set(&"local_transform".into(), &util::mat4x4_to_transform(&node.local_transform).to_variant());
    Some(data)
}

pub fn read_node_from_res(data: &ChumFile) -> Dictionary {
    let fmt = data.get_format();
    let mut dict = Dictionary::new();
    match read_node(&data.get_data_as_vec(), fmt) {
        Some(node) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"node".into(), &node.to_variant());
        }
        None => {
            godot_print!("read_node returned None");
            dict.set(&"exists".into(), &false.into());
        }
    }
    dict
}