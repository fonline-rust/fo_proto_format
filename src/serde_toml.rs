use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Proto {
    #[serde(alias = "Pid")]
    pub ProtoId: u16,
    //pub Type: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Protos {
    #[serde(alias = "Critter proto")]
    Proto: Vec<Proto>,
}

pub fn proto_from_toml(toml: &str) -> Result<Protos, toml::de::Error> {
    toml::de::from_str(toml)
}

fn proto_from_ini(ini: &str, filename: Option<&str>) -> Result<Protos, toml::de::Error> {
    let toml = crate::ini_to_toml::translate(ini, false, filename);
    proto_from_toml(&toml)
}

fn proto_from_file<P: AsRef<std::path::Path>>(path: P, lossy: bool) -> Result<Protos, toml::de::Error> {
    let filename = path.as_ref().file_name().and_then(|file| file.to_str());
    if lossy {
        let vec = std::fs::read(&path).unwrap();
        proto_from_ini(&String::from_utf8_lossy(&vec), filename)
    } else {
        let ini = std::fs::read_to_string(&path).unwrap();
        proto_from_ini(&ini, filename)
    }
}

use std::{
    path::Path,
    collections::BTreeMap,
};
pub fn build_btree() -> BTreeMap<u16, Proto> {
    let mut btree = BTreeMap::new();
    let list_path = Path::new("../test/FO4RP/proto/critters/critters.lst");
    //let list_path = Path::new("../test/FO4RP/proto/items/items.lst");
    let list = std::fs::read_to_string(list_path).unwrap();
    for path in list.lines() {
        let path = path.trim();
        if path.is_empty() {
            continue;
        }
        let file = list_path.with_file_name(path);
    //for file in std::fs::read_dir("../test/FO4RP/proto/critters/").unwrap().filter_map(|r| r.ok()) {
    //    let file = file.path();
        if !file.is_file() || file.extension() != Some("fopro".as_ref()) {
            panic!("Invalid file in fopro list: {:?}", file);
        }
        match proto_from_file(&file, true) {
            Ok(protos) => {
                for proto in protos.Proto {
                    let old = btree.insert(proto.ProtoId, proto);
                    if let Some(old) = old {
                        panic!("ProtoId {} collision!", old.ProtoId);
                    }
                }
            },
            Err(err) => {
                panic!("Error parsing {:?} file: {:?}", file, err);
            }
        }
    }
    btree
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fopro_toml_all() {
        for file in std::fs::read_dir("../test/FO4RP/proto/items/").unwrap().filter_map(|r| r.ok()) {
            let file = file.path();
            if !file.is_file() || file.extension() != Some("fopro".as_ref()) {
                continue;
            }
            println!("Parsing {:?}", file);
            if let Err(err) = proto_from_file(&file, true) {
                panic!("Error parsing {:?} file: {:?}", file, err);
            }
        }
    }

    #[test]
    fn grow_tree() {
        build_btree();
    }
}
