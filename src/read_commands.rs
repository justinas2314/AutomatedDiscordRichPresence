use std::collections::HashMap;

fn parse_line(string:&str) -> Vec<&str> {
    let mut prev_index = 0;
    let mut output = Vec::new();
    let length = match string.len() {
        0 => 0,
        x => x - 1
    };
    for i in 0..length {
        if prev_index > i {
            continue;
        }
        if &string[i..i + 2] == ",," {
            let val = &string[prev_index..i];
            if val.len() != 0 {
                output.push(val.trim());
            }
            prev_index = i + 2;
        }
    }
    output.push(&string[prev_index..].trim());
    output
}


pub fn main(contents: &str) -> HashMap<&str, (&str, &str, &str, &str, &str, &str)> {
    let mut dict = std::collections::HashMap::new();
    for i in contents.lines() {
        let output = parse_line(i);
        dict.insert(output[0],
                    (output[1], output[2], output[3], output[4], output[5], output[6]));
    }
    dict
}

pub fn order(text:&str) -> Vec<&str> {
    let mut output = Vec::new();
    for string in text.lines() {
        let length = match string.len() {
            0 => 0,
            x => x - 1
        };
        for i in 0..length {
            if &string[i..i + 2] == ",," {
                let val = &string[0..i];
                if val.len() != 0 {
                    output.push(val.trim());
                }
            }
        }
    }
    output
}

pub fn get_ordered(apps: Vec<(String, String)>, order: &Vec<&str>) -> Option<(String, String)> {
    for i in order {
        for j in &apps {
            if i == &j.1 {
                return Some((j.0.to_string(), j.1.to_string()));
            }
        }
    }
    None
}
