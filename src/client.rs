extern crate discord_rpc_client;


use discord_rpc_client::Client;
use std::collections::HashMap;


fn clear(client: &mut Client) {
    if let Err(e) = client.clear_activity() {
        println!("Failed to clear the current activity -> {}", e);
    }
}


fn function(client: &mut Client, vector: &Vec<&str>,
            raw_defaults: Option<&(&str, &str, &str, &str, &str, &str)>,
            bases: &HashMap<&str, (&str, &str, &str, &str, &str, &str)>) {
    let defaults = match raw_defaults {
        Some(x) => *x,
        None => ("", "", "", "", "", "") // causes an error later (that's intended lol)
    };
    let mut details = match vector.get(1) {
        Some(x) if defaults.0 == ".." => *x,
        Some(x) if match bases.get(defaults.0) {Some(x) if x.0 == ".." => true, _ => false} => *x,
        _ if bases.contains_key(defaults.0) => if bases[defaults.0].0.len() < 4 {"    "} else {bases[defaults.0].0},
        _ if defaults.0.len() < 4 => "    ",
        _ => defaults.0
    };
    let mut state = match vector.get(2) {
        _ if bases.contains_key(defaults.1) => if bases[defaults.1].1 == ".." {if details.len() >= 20 {""} else {"    "}} else {bases[defaults.1].1},
        _ if defaults.1 == ".." && details.len() >= 20 => "", // will be replaced
        Some(x) if defaults.1 == ".." => *x,
        _ if defaults.1.len() < 4 => "    ",
        _ => defaults.1
    };
    let mut large_text = match vector.get(3) {
        Some(x) if defaults.4 == ".." => *x,
        Some(x) if match bases.get(defaults.4) {Some(x) if x.4 == ".." => true, _ => false} => *x,
        _ if bases.contains_key(defaults.4) => bases[defaults.4].4,
        _ => defaults.4
    };
    let mut small_text = match vector.get(4) {
        Some(x) if defaults.5 == ".." => *x,
        Some(x) if match bases.get(defaults.5) {Some(x) if x.5 == ".." => true, _ => false} => *x,
        _ if bases.contains_key(defaults.5) => bases[defaults.5].5,
        _ => defaults.5
    };
    let large_image = match defaults.2 {
        x if bases.contains_key(x) => bases[x].2,
        "" => " ",
        x => x
    };
    let small_image = match defaults.3 {
        x if bases.contains_key(x) => bases[x].3,
        "" => " ",
        x => x
    };
    if state.len() == 0 && details.len() >= 20 {
        let output = split_line(details);
        details = output.0; // why couldn't I use deconstruction (tuple unpacking) here?
        state = output.1; // the compiler decided to complain lol
    }
    if state.len() >= 30 {
        state = "    ";
    }
    if details.len() >= 30 {
        details = "    ";
    }
    if large_text.len() > 128 {
        large_text = "";
    }
    if small_text.len() > 128 {
        small_text = "";
    }
    if let Err(e) = client.set_activity(|activity| activity
        .details(details)
        .state(state)
        .assets(|asset| {
            if large_text.len() != 0 {
                if small_text.len() != 0 {
                    asset.large_image(large_image)
                         .small_image(small_image)
                         .large_text(large_text)
                         .small_text(small_text)
                } else {
                    asset.large_image(large_image)
                        .small_image(small_image)
                        .large_text(large_text)
                }
            } else {
                if small_text.len() != 0 {
                    asset.large_image(large_image)
                        .small_image(small_image)
                        .small_text(small_text)
                } else {
                    asset.large_image(large_image)
                        .small_image(small_image)
                }
            }
        })) {
        println!("error occurred while setting an activity -> {}", e);
        println!("details -> \t ,{}; ,{}; ,{}; ,{}; ,{}; ,{};", details, state, large_image, small_image, large_text, small_text);
        println!("if your internet connection is stable this might be a bug");
    }
}


fn split_line(line: &str) -> (&str, &str) {
    let mut smaller_split_index = 0;
    let mut bigger_split_index = 0;
    for (i, j) in line.chars().enumerate() {
        if j == ' ' {
            if i >= 30 { // strings can't be longer than that
                break;
            }
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
            bases: &HashMap<&str, (&str, &str, &str, &str, &str, &str)>, parsed_input: Vec<&str>) {
    // ',,' seperates the values
    // spaces between the values are stripped
    // '..' skips the value and uses the default one
    // ',, ,,' leaves the value as ''
    // ',,,' splits and writes a comma (,)
    // ',,,,' <- this is retarded and messes the code up don't do this
    // you can create a conditional list '[string1,, string2]'
    // every value inside this list must be included to match
    // ',,--' makes sure that the following value is not included
    // ',, --' does nothing, use this if you want '--' in your string that must match
    // the details and state values cannot be less than 4 or more than 29
    // if the length is less than 4 it will be automatically replaced with "    "
    // if it is more than 29 an error will occur
    match parsed_input.get(0){
        Some(&"clear") => clear(&mut client),
        Some(x) => function(&mut client,
                            &parsed_input,
                            presets.get(*x),
                            bases),
        _ => ()
    }
}
