pub fn translate(ini: &str, with_comments: bool, filename: Option<&str>) -> String {
    let mut toml = String::with_capacity(ini.len() + ini.len() / 20);
    for (line_num, mut line) in ini.lines().enumerate() {
        line = line.trim_start_matches('\u{feff}').trim();
        let (line, comment) = split_until(line, '#');

        if line.is_empty() {
            continue;
        }

        if line.starts_with('[') {
            if !line.ends_with(']') || line.len() < 3 {
                panic!(
                    "Malformed section! {:?}:{:?} => {:?}",
                    filename, line_num, line
                );
            }
            toml.push_str("[[\"");
            //replace_in_place(line, &mut)
            toml.push_str(&line[1..line.len() - 1]);
            toml.push_str("\"]]")
        } else {
            let (key, val) = split_until(line, '=');
            if let Some(val) = val {
                toml.push_str(&key.replace('.', "_"));
                toml.push('=');
                if val == "0" || !val.starts_with('0')
                    && val.len() > 0
                    && val.len() < 20
                    && val
                        .bytes()
                        .all(|byte| byte == b'-' || (b'0' <= byte && byte <= b'9'))
                {
                    //let val_u64: Result<i64, _> = val.parse();
                    //if val_u64.is_ok(){
                    toml.push_str(val);
                } else {
                    toml.push('"');
                    replace_in_place(val, &mut toml, '\\', "\\\\");
                    toml.push('"');
                }
            } else {
                panic!(
                    "Line without equals sign! {:?}:{:?} => {:?}",
                    filename, line_num, line
                );
            }
        }

        if with_comments {
            if let Some(comment) = comment {
                toml.push_str(" #");
                toml.push_str(comment);
            }
        }

        toml.push('\n');
    }
    toml
}

fn split_until(s: &str, delimeter: char) -> (&str, Option<&str>) {
    let mut split = s.splitn(2, delimeter);
    (
        split.next().unwrap().trim_end(),
        split.next().map(|s| s.trim_start()),
    )
}

fn replace_in_place(original: &str, result: &mut String, from: char, to: &str) {
    let mut last_end = 0;
    for (start, part) in original.match_indices(from) {
        result.push_str(unsafe { original.get_unchecked(last_end..start) });
        result.push_str(to);
        last_end = start + part.len();
    }
    result.push_str(unsafe { original.get_unchecked(last_end..original.len()) });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fopro_generic() {
        let ini = std::fs::read_to_string("../FO4RP/proto/items/generic.fopro").unwrap();
        let _toml = translate(&ini, true, None);
        //println!("{}", toml);
    }
    #[test]
    fn fopro_food() {
        let ini = std::fs::read_to_string("../FO4RP/proto/items/food.fopro").unwrap();
        let _toml = translate(&ini, true, None);
        //println!("{}", toml);
    }
}
