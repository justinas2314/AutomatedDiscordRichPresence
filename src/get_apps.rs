// (full, base)
pub fn main(order: &Vec<&str>) -> Vec<(String, String)> {
    // the part where I request windows to give me the list
    // this is also the slowest part
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
    let lowercase_line = line.to_lowercase();
    for i in line.trim().chars() {
        match i {
            ',' => {string.clear();},
            x => {string.push(x);}
        }
    }
    for i in order {
        let mut contains = true;
        for j in split(i) {
            if j.len() > 2 && &j[0..1] == "-" {
                if lowercase_line.contains(j[2..]
                    .replace("[", "")
                    .replace("]", "")
                    .trim()) {
                    contains = false;
                    break;
                }
            } else if j.len() > 2 && &j[0..1] == "\\" &&  !lowercase_line.contains(j
                .replacen("\\", "", 1)
                .replace("[", "")
                .replace("]", "")
                .trim()){

            } else if !lowercase_line.contains(j
                .replace("[", "")
                .replace("]", "")
                .trim()) {
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
    (output_full.replace("N/A", "").replace("\"", ""), output_base)
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
                output.push(buffer.trim().to_string());
                buffer.clear();
                escape = false
            },
            x => {
                buffer.push(x);
                escape = false
            }
        }
    }
    output.push(buffer.trim().to_string());
    output
}
