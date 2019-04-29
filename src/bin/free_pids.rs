use fo_proto_format::build_btree;

const MAX_ITEM_PROTOTYPES: u16 = 30000;

fn main() {
    let btree = build_btree();
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
    println!("Free PIDs:");
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
