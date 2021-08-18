use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, path::Path};
use std::fmt::Debug;

pub trait Proto: Debug + DeserializeOwned + Debug {
    fn proto_id(&self) -> u16;
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(bad_style)]
pub struct ProtoItem {
    #[serde(alias = "Pid")]
    pub ProtoId: u16,
    pub Type: u8,
    #[serde(alias = "PicMapName")]
    pub PicMap: String,
    pub Flags: Option<u32>,
    pub Grid_Type: Option<u8>,
}
impl Proto for ProtoItem{
    fn proto_id(&self) -> u16 {
        self.ProtoId
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(bad_style)]
pub struct ProtoCritter {
    #[serde(alias = "Pid")]
    pub ProtoId: u16,
}
impl Proto for ProtoCritter{
    fn proto_id(&self) -> u16 {
        self.ProtoId
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(bad_style)]
pub struct Protos<T> {
    #[serde(alias = "Critter proto")]
    Proto: Vec<T>,
}

pub fn proto_from_toml<T: Proto>(toml: &str) -> Result<Protos<T>, toml::de::Error> {
    toml::de::from_str(toml)
}

fn proto_from_ini<T: Proto>(ini: &str, filename: Option<&str>) -> Result<Protos<T>, toml::de::Error> {
    let toml = crate::ini_to_toml::translate(ini, false, filename);
    let res = proto_from_toml(&toml);
    /*if let Err(err) = &res {
        let line = err.line_col().unwrap().0;
        let debug: Vec<_> = toml.lines().skip(line.saturating_sub(3)).take(7).collect();
        std::fs::write("../ini_to_toml.txt", &toml).unwrap();
        println!("lines:\n{:?}", debug);
    }*/
    res
}

fn proto_from_file<T: Proto, P: AsRef<std::path::Path>>(
    path: P,
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

pub fn build_btree<T: Proto, P: AsRef<Path>>(list_path: P) -> BTreeMap<u16, T> {
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
        match proto_from_file::<T, _>(&file, true) {
            Ok(protos) => {
                for proto in protos.Proto {
                    let old = btree.insert(proto.proto_id(), proto);
                    if let Some(old) = old {
                        panic!("ProtoId {} collision!", old.proto_id());
                    }
                }
            }
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
        for file in std::fs::read_dir("../FO4RP/proto/items/")
            .unwrap()
            .filter_map(|r| r.ok())
        {
            let file = file.path();
            if !file.is_file() || file.extension() != Some("fopro".as_ref()) {
                continue;
            }
            println!("Parsing {:?}", file);
            if let Err(err) = proto_from_file::<ProtoItem, _>(&file, true) {
                panic!("Error parsing {:?} file: {:?}", file, err);
            }
        }
    }

    #[test]
    fn tirs_2007() {
        let btree = build_btree::<ProtoItem, _>("../FO4RP/proto/items/items.lst");
        let tirs = btree.get(&2007).expect("tirs 2007");
        println!("tirs: {:?}", tirs);
    }
}
