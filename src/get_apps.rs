use std::collections::HashMap;


// (full, base)
pub fn main(commands: &HashMap<&str, (&str, &str, &str, &str, &str, &str)>) -> Vec<(String, String)> {
    // the part where I request windows to give me the list
    // of all tasks is so slow that i don't need to make the code sleep
    // this part only works on windows
    let var = std::process::Command::new("tasklist")
        .arg("/v")
        .arg("/FO")
        .arg("CSV")
        .output()
        .expect("Failed to get the running windows")
        .stdout;
    let s = String::from_utf8(var)
        .unwrap();
    let mut data = Vec::new();
    for i in s.lines() {
        let (full, base) = parse_line(commands, i);
        if full.len() != 0 {
            data.push((full, base));
        }
    }
    data
}


// (full name, base name)
fn parse_line(commands: &HashMap<&str, (&str, &str, &str, &str, &str, &str)>, line: &str) -> (String, String) {
    let mut string = String::new();
    let mut output_full = String::new();
    let mut output_base = String::new();
    for i in line.trim().chars() {
        match i {
            ',' => {string.clear();},
            x => {string.push(x);}
        }
    }
    for i in commands.keys() {
        if string.to_lowercase().contains(*i) {
            output_full.push_str(&string);
            output_base.push_str(*i);
            break;
        }
    }
    (output_full.replace("\"", ""), output_base)
}
