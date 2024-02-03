use std::{path::{PathBuf, Path}, ffi::OsStr, io::Result as IoResult};

use anyhow::{Result, Context};
use fo_proto_format::*;

fn main() {
    if let Err(err) = to_json() {
        panic!("{:#}", err);
    }
}

fn to_json() -> Result<()> {
    let dir = dir("PROTO_PATH", "proto_path.cfg")
        .unwrap_or("../FO4RP/proto".into())
        .canonicalize()?;

    let items_path = path_to(dir, "items/items.lst")?;
    let btrees = build_btree_per_file::<ProtoItem>(items_path);
    for (file, btree) in btrees {
        let vec: Vec<_> = btree.values().collect();
        let json = serde_json::to_string_pretty(&vec).with_context(|| file.display().to_string())?;
        let mut path = PathBuf::from("./output/");
        path.push(file.file_name().unwrap());
        path.set_extension("json");
        std::fs::write(path, json).expect("Can't write result to file");
    }
    Ok(())
}

fn path_to(prefix: impl AsRef<Path>, to: impl AsRef<Path>) -> IoResult<PathBuf> {
    let mut path = prefix.as_ref().to_owned();
    path.extend(to.as_ref().iter());
    path.canonicalize()
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
