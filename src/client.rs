extern crate discord_rpc_client;


use discord_rpc_client::Client;
use std::collections::HashMap;


fn clear(client: &mut Client) {
    if let Err(e) = client.clear_activity() {
        println!("error occurred while clearing the activity \n{}", e);
    }
}


fn function(client: &mut Client, vector: &Vec<&str>,
            raw_defaults: Option<&(&str, &str, &str, &str, &str, &str)>) {
    let defaults = match raw_defaults {
        Some(x) => *x,
        None => ("", "", "", "", "", "") // causes an error later (that's intended lol)
    };
    let mut details = match vector.get(1) {
        Some(x) if defaults.0 == ".." => *x,
        Some(_) if defaults.0.len() < 4 => "    ",
        Some(_) => defaults.0,
        None if defaults.0.len() < 4 => "    ",
        None => defaults.0
    };
    let mut state = match vector.get(2) {
        Some(_) if defaults.1 == ".." && details.len() >= 20 => "", // will be replaced
        Some(x) if defaults.1 == ".." => *x,
        Some(_) if defaults.1.len() < 4 => "    ",
        Some(_) => defaults.1,
        None if defaults.1 == ".." && details.len() >= 20 => "", // will also be replaced
        None if defaults.1.len() < 4 => "    ",
        None => defaults.1
    };
    let mut large_text = match vector.get(3) {
        Some(_) if defaults.4 == ".." => "..",
        Some(x) => *x,
        None => ""
    };
    let mut small_text = match vector.get(4) {
        Some(_) if defaults.5 == ".." => "..",
        Some(x) => *x,
        None => ""
    };
    if state.len() == 0 && details.len() >= 20 {
        let output = split_line(details);
        details = output.0; // why couldn't I use deconstruction (tuple unpacking) here?
        state = output.1; // the compiler decided to complain lol
    }
    if large_text == ".." {
        large_text = match vector.get(1) {
            Some(x) => *x,
            None => ""
        };
    }
    if small_text == ".." {
        small_text = match vector.get(1) {
            Some(x) => *x,
            None => ""
        };
    }
    // if let Err(e) = client.clear_activity() {
    // 	println!("an error occured while clearing the activity \n {}", e);
    // }
    // need a better closure here
    if let Err(e) = client.set_activity(|activity| activity
        .details(details)
        .state(state)
        .assets(|asset| {
            if large_text.len() != 0 {
                if small_text.len() != 0 {
                    asset.large_image(defaults.2)
                         .small_image(defaults.3)
                         .large_text(large_text)
                         .small_text(small_text)
                } else {
                    asset.large_image(defaults.2)
                        .small_image(defaults.3)
                        .large_text(large_text)
                }
            } else {
                if small_text.len() != 0 {
                    asset.large_image(defaults.2)
                        .small_image(defaults.3)
                        .small_text(small_text)
                } else {
                    asset.large_image(defaults.2)
                        .small_image(defaults.3)
                }
            }
        })) {
        println!("error occurred while setting an activity \n {}", e);
        println!("details -> \t ,{}; ,{}; ,{}; ,{};", state, details, defaults.2, defaults.3);
    }
}


fn split_line(line: &str) -> (&str, &str) {
    let mut smaller_split_index = 0;
    let mut bigger_split_index = 0;
    for (i, j) in line.chars().enumerate() {
        if j == ' ' {
            if i > 20 {
                bigger_split_index = i;
                break;
            } else {
                smaller_split_index = i;
            }
        }
    }
    let split_index = match (smaller_split_index, bigger_split_index) {
        (x, y) if (20 - x as i32) * (20 - x as i32) > (20 - y as i32) * (20 - y as i32) => y,
        (x, _) => x,
    };
    match (&line[0..split_index], &line[split_index + 1..]) {
        (x, y) if x.len() < 4 || y.len() < 4 || split_index == 0 => (line, "    "),
        (x, y) => (x, y)
    }
}


pub fn main(mut client: &mut Client, presets: &HashMap<&str, (&str, &str, &str, &str, &str, &str)>,
            parsed_input: Vec<&str>) {
    // ',,' seperates the values
    // spaces between the values are stripped
    // '..' skips the value and uses the default one
    // ',, ,,' leaves the value as ''
    // ',,,' splits and writes a comma (,)
    // ',,,,' <- this is retarded and messes the code up don't do this
    match parsed_input.get(0){
        Some(&"clear") => clear(&mut client),
        Some(x) => function(&mut client,
                            &parsed_input,
                            presets.get(*x)),
        _ => ()
    }
}
