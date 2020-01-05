use fo_proto_format::*;
use std::ffi::OsStr;
use std::fmt::Write;
use std::path::{Path, PathBuf};

const MAX_ITEM_PROTOTYPES: u16 = 30000;

fn main() {
    match free_pids() {
        Ok(buffer) => {
            std::fs::write("./free_pids.txt", buffer).expect("Can't write result to file");
        }
        Err(err) => {
            let err_string = format!("Error: {}", err);
            std::fs::write("./free_pids.txt", &err_string).expect("Can't write error to file");
            panic!("{:?}", err);
        }
    }
}

fn free_pids() -> Result<String, Box<dyn std::error::Error>> {
    let dir = dir("PROTO_PATH", "proto_path.cfg")
        .unwrap_or("../FO4RP/proto".into())
        .canonicalize()?;
    let mut buffer = String::with_capacity(2048);

    banner(&mut buffer, "Free PIDs for items")?;
    let items_path = path_to(&dir, "items/items.lst")?;
    ranges_for::<ProtoItem, _, _>(&mut buffer, items_path)?;

    buffer.write_char('\n')?;
    banner(&mut buffer, "Free PIDs for critters")?;
    let critters_path = path_to(&dir, "critters/critters.lst")?;
    ranges_for::<ProtoCritter, _, _>(&mut buffer, critters_path)?;
    Ok(buffer)
}

fn banner(f: &mut impl Write, text: &str) -> std::fmt::Result {
    writeln!(
        f,
        "======================\n\
         {}\n\
         ======================\n",
        text
    )
}

fn path_to(prefix: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let mut path = prefix.as_ref().to_owned();
    path.extend(to.as_ref().iter());
    path.canonicalize()
}

fn ranges_for<T: Proto, P: AsRef<Path>, W: Write>(f: &mut W, path: P) -> std::fmt::Result {
    let btree = build_btree::<T, _>(path);
    let mut first_free = 1;
    let mut inclusive_ranges = Vec::with_capacity(128);
    for (&key, _value) in &btree {
        if key - first_free > 0 {
            inclusive_ranges.push((first_free, key - 1));
        }
        first_free = key + 1;
    }
    if first_free < MAX_ITEM_PROTOTYPES {
        inclusive_ranges.push((first_free, MAX_ITEM_PROTOTYPES - 1));
    }
    for (from, to) in inclusive_ranges {
        if from == to {
            writeln!(f, "{}", from)?;
        } else if from + 1 == to {
            writeln!(f, "{}, {}", from, to)?;
        } else {
            let count = to - from + 1;
            writeln!(f, "{}-{} ({} PIDs)", from, to, count)?;
        }
    }
    Ok(())
}

pub fn dir<P1: AsRef<OsStr>, P2: AsRef<Path>>(env: P1, file: P2) -> Option<PathBuf> {
    let env = std::env::var_os(env);
    if let Some(path) = env.and_then(|env| Path::new(&env).canonicalize().ok()) {
        Some(path)
    } else if let Ok(path) =
        std::fs::read_to_string(file).and_then(|env| Path::new(&env).canonicalize())
    {
        Some(path)
    } else {
        None
    }
}
