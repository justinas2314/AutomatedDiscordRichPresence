use std::collections::HashMap;


// lower means only read lowercase lines
fn parse_line(string:&str, lower: bool) -> Option<Vec<&str>> {
    if lower == ("ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(&string[0..1])) {
        return None;
    }
    let mut prev_index = 0;
    let mut output = Vec::new();
    let length = match string.len() {
        0 => 0,
        x => x - 1
    };
    let mut skip = false;
    for i in 0..length {
        if prev_index > i {
            continue;
        }
        match &string[i..i + 1] {
            "[" if i != 0 && &string[i - 1..i] == "\\" => {skip = false;},
            "[" => {skip = true;},
            "]" if i != 0 && &string[i - 1..i] == "\\" => {skip = true;},
            "]" => {skip = false;},
            _ => ()
        }
        if &string[i..i + 2] == ",," && !skip {
            let val = &string[prev_index..i];
            if val.len() != 0 {
                output.push(val.trim());
            }
            prev_index = i + 2;
        }
    }
    output.push(&string[prev_index..].trim());
    Some(output)
}


pub fn main(contents: &str) -> HashMap<&str, (&str, &str, &str, &str, &str, &str)> {
    let mut dict = HashMap::new();
    for i in contents.lines() {
        if let Some(output) = parse_line(i, true) {
            dict.insert(output[0],
                        (rp(&output, 1),
                            rp(&output, 2),
                            rp(&output, 3),
                            rp(&output, 4),
                            rp(&output, 5),
                            rp(&output,6)));
        }
    }
    dict
}


pub fn order(text: &str) -> Vec<&str> {
    let mut output = Vec::new();
    for string in text.lines() {
        let length = match string.len() {
            0 => 0,
            x => x - 1
        };
        let mut skip = false;
        for i in 0..length {
            match &string[i..i + 1] {
                "[" if i != 0 && &string[i - 1..i] == "\\" => {skip = false;},
                "[" => {skip = true;},
                "]" if i != 0 && &string[i - 1..i] == "\\" => {skip = true;},
                "]" => {skip = false;},
                _ => ()
            }
            if &string[i..i + 2] == ",," && !skip {
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


pub fn bases(contents: &str) -> HashMap<&str, (&str, &str, &str, &str, &str, &str)> {
    let mut dict = HashMap::new();
    for i in contents.lines() {
        if let Some(output) = parse_line(i, false) {
            dict.insert(output[0],
                        (rp(&output, 1),
                            rp(&output, 2),
                            rp(&output, 3),
                            rp(&output, 4),
                            rp(&output, 5),
                            rp(&output,6)));
        }
    }
    dict
}


// gets the value from a vector and if the value doesn't exist returns ""
fn rp<'a>(input: &Vec<&'a str>, index: usize) -> &'a str {
    match input.get(index) {
        Some(x) => *x,
        None => ""
    }
}