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
            "!" => {
                if buffer.len() != 0 {
                    let (name, dict) = parse(&buffer, &commands);
                    commands.insert(name, dict);
                }
                buffer.clear();
                buffer.push(&line[1..]);
            }
            "[" => {
                // should always be true
                if let Some(x) = line.find("]") {
                    order.push(&line[0..=x]);
                }
                if buffer.len() != 0 {
                    let (name, dict) = parse(&buffer, &commands);
                    commands.insert(name, dict);
                }
                buffer.clear();
                buffer.push(line);
            },
            _ => {buffer.push(line)}
        }
    }
    if buffer.len() != 0 {
        let (name, dict) = parse(&buffer, &commands);
        commands.insert(name, dict);
    }
    (commands, order)
}


fn parse(lines: &Vec<&str>, commands: &HashMap<String, HashMap<String, String>>
    ) -> (String, HashMap<String, String>) {
    // this is a mess
    // but if it works it works
    let mut dict = HashMap::new();
    let mut name = String::new();
    let mut parent = String::new();
    for line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }
        if let "[" = &line[0..1] {
            let mut closed = false;
            let mut arrow = false;
            let mut buffer_char = None;
            for i in line.chars() {
                match buffer_char {
                    None => {buffer_char = Some(i); continue},
                    Some(_) => ()
                }
                match (buffer_char, i) {
                    (Some('\\'), ']') => (),
                    (Some(x), ']') if arrow => {
                        parent.push(x);
                        parent.push(']');
                    },
                    (Some(x), _) if arrow => {parent.push(x)},
                    (Some('<'), '-') if closed => {arrow = true},
                    (Some(x), ']') if !closed => {
                        name.push(x);
                        name.push(']');
                        closed = true;
                    },
                    (Some(x), _) if !closed => {name.push(x)},
                    _ => ()
                }
                buffer_char = Some(i);
            }
            name = name.trim().to_string();
            if parent.len() != 0 {
                parent = parent[1..].trim().to_string();
            }
            if let Some(x) = commands.get(parent.trim()) {
                for (key, value) in x{
                    dict.insert(key.to_string(), value.to_string());
                }
            }
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
