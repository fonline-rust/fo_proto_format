use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub trait Proto: Debug + DeserializeOwned + Debug {
    fn proto_id(&self) -> u16;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProtoItem {
    #[serde(rename = "ProtoId", alias = "Pid")]
    pub proto_id: u16,
    #[serde(rename = "Type")]
    pub ty: u8,
    #[serde(rename = "PicMap", alias = "PicMapName")]
    pub pic_map: String,
    #[serde(default, rename = "PicInv", alias = "PicInvName")]
    pub pic_inv: Option<String>,
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,
    #[serde(rename = "Grid_Type")]
    pub grid_type: Option<u8>,
    #[serde(rename = "Weapon_Perk")]
    pub weapon_perk: Option<u32>,
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}
impl Proto for ProtoItem {
    fn proto_id(&self) -> u16 {
        self.proto_id
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(bad_style)]
pub struct ProtoCritter {
    #[serde(rename = "ProtoId", alias = "Pid")]
    pub proto_id: u16,
}
impl Proto for ProtoCritter {
    fn proto_id(&self) -> u16 {
        self.proto_id
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Protos<T> {
    #[serde(rename = "Proto", alias = "Critter proto")]
    values: Vec<T>,
}

pub fn proto_from_toml<T: Proto>(toml: &str) -> Result<Protos<T>, toml::de::Error> {
    toml::de::from_str(toml)
}

fn proto_from_ini<T: Proto>(
    ini: &str,
    filename: Option<&str>,
) -> Result<Protos<T>, toml::de::Error> {
    let toml = crate::ini_to_toml::translate(ini, false, filename);
    proto_from_toml(&toml)
    /*if let Err(err) = &res {
        let line = err.line_col().unwrap().0;
        let debug: Vec<_> = toml.lines().skip(line.saturating_sub(3)).take(7).collect();
        std::fs::write("../ini_to_toml.txt", &toml).unwrap();
        println!("lines:\n{:?}", debug);
    }*/
}

fn proto_from_file<T: Proto>(
    path: impl AsRef<std::path::Path>,
    lossy: bool,
) -> Result<Protos<T>, toml::de::Error> {
    let filename = path.as_ref().file_name().and_then(|file| file.to_str());
    if lossy {
        let vec = std::fs::read(&path).unwrap();
        proto_from_ini(&String::from_utf8_lossy(&vec), filename)
    } else {
        let ini = std::fs::read_to_string(&path).unwrap();
        proto_from_ini(&ini, filename)
    }
}

pub fn build_btree_per_file<T: Proto>(
    list_path: impl AsRef<std::path::Path>,
) -> HashMap<PathBuf, BTreeMap<u16, T>> {
    let list = std::fs::read_to_string(list_path.as_ref()).unwrap();
    let mut res = HashMap::new();
    for path in list.lines() {
        let path = path.trim();
        if path.is_empty() {
            continue;
        }
        let file = list_path.as_ref().with_file_name(path);
        //for file in std::fs::read_dir("../test/FO4RP/proto/critters/").unwrap().filter_map(|r| r.ok()) {
        //    let file = file.path();
        if !file.is_file() || file.extension() != Some("fopro".as_ref()) {
            panic!("Invalid file in fopro list: {:?}", file);
        }
        let mut btree = BTreeMap::new();
        match proto_from_file::<T>(&file, true) {
            Ok(protos) => {
                for proto in protos.values {
                    let old = btree.insert(proto.proto_id(), proto);
                    if let Some(old) = old {
                        panic!("ProtoId {} collision!", old.proto_id());
                    }
                }
            }
            Err(err) => {
                panic!("Error parsing file {:?}: {:#}", file, err);
            }
        }
        res.insert(file, btree);
    }
    res
}

pub fn build_btree<T: Proto>(list_path: impl AsRef<std::path::Path>) -> BTreeMap<u16, T> {
    let mut btree = BTreeMap::new();
    let list = std::fs::read_to_string(list_path.as_ref()).unwrap();
    for path in list.lines() {
        let path = path.trim();
        if path.is_empty() {
            continue;
        }
        let file = list_path.as_ref().with_file_name(path);
        //for file in std::fs::read_dir("../test/FO4RP/proto/critters/").unwrap().filter_map(|r| r.ok()) {
        //    let file = file.path();
        if !file.is_file() || file.extension() != Some("fopro".as_ref()) {
            panic!("Invalid file in fopro list: {:?}", file);
        }
        match proto_from_file::<T>(&file, true) {
            Ok(protos) => {
                for proto in protos.values {
                    let old = btree.insert(proto.proto_id(), proto);
                    if let Some(old) = old {
                        panic!("ProtoId {} collision!", old.proto_id());
                    }
                }
            }
            Err(err) => {
                panic!("Error parsing file {:?}: {:#}", file, err);
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
        for file in std::fs::read_dir("../FO4RP/proto/items/")
            .unwrap()
            .filter_map(|r| r.ok())
        {
            let file = file.path();
            if !file.is_file() || file.extension() != Some("fopro".as_ref()) {
                continue;
            }
            println!("Parsing {:?}", file);
            if let Err(err) = proto_from_file::<ProtoItem>(&file, true) {
                panic!("Error parsing {:?} file: {:?}", file, err);
            }
        }
    }

    #[test]
    fn tirs_2007() {
        let btree = build_btree::<ProtoItem>("../FO4RP/proto/items/items.lst");
        let tirs = btree.get(&2007).expect("tirs 2007");
        println!("tirs: {:?}", tirs);
    }
}
