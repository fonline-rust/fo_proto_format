#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fopro_generic_cini() {
        let mut proto = Proto::default();
        proto.parse("../FO4RP/proto/items/generic.fopro");
    }
}

use cini::{Callback, CallbackKind, Ini};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Proto(ProtoInner);

impl Proto {
    pub fn parse<P: AsRef<std::path::Path>>(&mut self, path: P) {
        let ini = std::fs::read_to_string(path).unwrap();
        self.0.parse_str(&ini).unwrap();
        self.0.insert();
    }
    pub fn used(&self) -> &BTreeMap<u16, i32> {
        &self.0.used
    }
}

#[derive(Default)]
struct ProtoInner {
    used: BTreeMap<u16,i32>,
    current: Option<(u16, i32)>,
}

impl ProtoInner {
    fn insert(&mut self) {
        match self.current {
            None => {},
            Some((0, _)) => {
                panic!("Proto without ProtoId");
            },
            Some((_, 0)) => {
                panic!("Proto without type");
            },
            Some((key, value))  => {
                let old = self.used.insert(key, value);
                assert!(old.is_none());
            }
        };
        self.current = None;
    }
}

impl Ini for ProtoInner {
    type Err = String;

    fn callback(&mut self, cb: Callback) -> Result<(), Self::Err> {
        match cb.kind {
            CallbackKind::Section(section) => {
                //println!("\nSection: {:?}", section);
                self.insert();
            },
            CallbackKind::Directive(section, key, value) => {
                //println!("{:?} => {:?}", key, value);
                if key == "ProtoId" {
                    let pid: u16 = value.unwrap().parse().unwrap();
                    self.current = Some(match self.current {
                        None => (pid, 0),
                        Some((0,ty)) => (pid,ty),
                        _ => panic!("Two 'ProtoId' per section, {:?}", cb.line_number)
                    });
                } else if key == "Type" {
                    let ty: i32 = value.unwrap().parse().unwrap();
                    self.current = Some(match self.current {
                        None => (0, ty),
                        Some((pid,0)) => (pid,ty),
                        _ => panic!("Two 'Type' per section, {:?}", cb.line_number)
                    });
                }
            }
        };
        Ok(())
    }
}
