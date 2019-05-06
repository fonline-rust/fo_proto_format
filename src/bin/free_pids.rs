use fo_proto_format::build_btree;
use std::path::Path;

const MAX_ITEM_PROTOTYPES: u16 = 30000;

fn main() {
    println!("\nFree PIDs for items:");
    ranges_for("../FO4RP/proto/items/items.lst");
    println!("\nFree PIDs for critters:");
    ranges_for("../FO4RP/proto/critters/critters.lst");
}

fn ranges_for<P: AsRef<Path>>(path: P) {
    let btree = build_btree(path);
    let mut first_free = 1;
    let mut inclusive_ranges = Vec::with_capacity(128);
    for (&key, _value) in &btree {
        if key-first_free > 0 {
            inclusive_ranges.push((first_free, key-1));
        }
        first_free = key+1;
    }
    if first_free < MAX_ITEM_PROTOTYPES {
        inclusive_ranges.push((first_free, MAX_ITEM_PROTOTYPES-1));
    }
    for (from, to) in inclusive_ranges {
        if from == to {
            println!("{}", from);
        } else if from+1 == to {
            println!("{}, {}", from, to);
        } else {
            let count = to-from+1;
            println!("{}-{} ({} PIDs)", from, to, count);
        }
    }
}
