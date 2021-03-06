// (full, base)
pub fn main(order: &Vec<&str>) -> Vec<(String, String)> {
    // the part where I request windows to give me a list of all tasks
    // this is also the slowest part
    // and it only works on windows (i think ??)
    let var = std::process::Command::new("tasklist")
        .arg("/v")
        .arg("/FO")
        .arg("CSV")
        .output()
        .expect("Failed to get the running windows")
        .stdout;
    let s = String::from_utf8_lossy(&var);
    let mut data = Vec::new();
    for i in s.lines() {
        let (full, base) = parse_line(&order, i);
        if full.len() != 0 {
            data.push((full, base));
        }
    }
    data
}


// (full name, base name)
fn parse_line(order: &Vec<&str>, line: &str) -> (String, String) {
    let mut string = String::new();
    let mut output_full = String::new();
    let mut output_base = String::new();
    let mut escape = false;
    let lowercase_line = line.to_lowercase();
    for i in line.trim().chars() {
        match i {
            '"' => {escape = !escape;},
            ',' if !escape => {string.clear();},
            x => {string.push(x);}
        }
    }
    for i in order {
        let mut contains = true;
        for j in split(i) {
            if let Some(0) = j.find("-") {
                if lowercase_line.contains(j[1..].trim()) {
                    contains = false;
                    break;
                }
            } else if Some(0) == j.find("\\") && !lowercase_line.contains(j[1..].trim()){
                contains = false;
                break;
            } else if !lowercase_line.contains(j.trim()) {
                contains = false;
                break;
            }
        }
        if contains {
            output_full.push_str(&string);
            output_base.push_str(i);
            break;
        }
    }
    (output_full.replace("N/A", ""), output_base)
}


fn split(text: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut escape = false;
    let mut buffer = String::new();
    for i in text.chars() {
        match i {
            '\\' if !escape => {
                escape = true
            },
            ',' if !escape => {
                output.push(buffer.to_lowercase().trim().to_string());
                buffer.clear();
                escape = false
            },
            '[' if !escape => (),
            ']' if !escape => (),
            x => {
                buffer.push(x);
                escape = false
            }
        }
    }
    output.push(buffer.to_lowercase().trim().to_string());
    output
}
