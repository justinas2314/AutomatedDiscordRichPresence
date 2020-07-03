use std::collections::HashMap;

pub fn main(text: &str) -> (HashMap<String, HashMap<String, String>>, Vec<&str>) {
    let mut buffer = Vec::new();
    let mut commands = HashMap::new();
    let mut order = Vec::new();
    for line in text.lines() {
        if line.trim().len() == 0 {
            continue;
        }
        match &line[0..1] {
            ";" => continue,
            "[" => {
                order.push(line);
                if buffer.len() != 0 {
                    let (name, dict) = parse(&buffer);
                    commands.insert(name, dict);
                }
                buffer.clear();
                buffer.push(line);
            },
            _ => {buffer.push(line)}
        }
    }
    if buffer.len() != 0 {
        let (name, dict) = parse(&buffer);
        commands.insert(name, dict);
    }
    (commands, order)
}


fn parse(lines: &Vec<&str>) -> (String, HashMap<String, String>) {
    let mut dict = HashMap::new();
    let mut name = String::new();
    for line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }
        if &line[0..1] == "[" {
            name = line.trim().to_string();
        } else {
            let (key, value) = get_kv(line);
            dict.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    (name, dict)
}


fn get_kv(line: &str) -> (String, String) {
    let mut key = String::new();
    let mut value = String::new();
    let mut switched = false;
    let mut escape = false;
    for i in line.chars() {
        match i {
            '\\' if !escape => {escape = true},
            '=' if !escape => {switched = true},
            x if !switched => {key.push(x); escape = false},
            x if switched => {value.push(x); escape = false},
            _ => ()
        }
    }
    (key, value)
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

