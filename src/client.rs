extern crate discord_rpc_client;


use discord_rpc_client::Client;
use std::collections::HashMap;


fn clear(client: &mut Client) {
    if let Err(e) = client.clear_activity() {
        println!("Failed to clear the current activity -> {}", e);
    }
}


fn function(client: &mut Client, vector: &Vec<&str>,
            raw_defaults: Option<&HashMap<String, String>>) {
    let defaults = match raw_defaults {
        Some(x) => x,
        None => return // causes an error later (that's intended lol)
    };
    // i don't know what this does anymore
    let mut details: &str = match defaults.get("details") {
        Some(x) if x == ".." => vector[1],
        Some(x) if x.len() < 4 || x.len() > 29 => "    ",
        Some(x) => x,
        None => "    "
    };
    let mut state: &str = match defaults.get("state") {
        Some(x) if x == ".." && details.len() >= 20 => "", // will be replaced
        Some(x) if x.len() < 4 || x.len() > 29 => "    ",
        Some(x) => x,
        None => "    "
    };
    let large_text: &str = match defaults.get("large_text") {
        Some(x) if x == ".." => if vector[1].len() > 128 {too_long(vector[1])} else {vector[1]},
        Some(x) if x.len() > 128 => too_long(x),
        Some(x) => x,
        None => "" // will be replaced
    };
    let small_text = match defaults.get("small_text") {
        Some(x) if x == ".." => if vector[1].len() > 128 {too_long(vector[1])} else {vector[1]},
        Some(x) if x.len() > 128 => too_long(x),
        Some(x) => x,
        None => "" // will be replaced
    };
    // i think 0 length strings are not allowed
    // also if the image name is incorrect no error will occur
    let large_image: &str = match defaults.get("large_image") {
        Some(x) if x == ".." => " ",
        Some(x) if x == "" => " ",
        Some(x) => x,
        None => " "
    };
    // same here
    let small_image: &str = match defaults.get("small_image") {
        Some(x) if x == ".." => " ",
        Some(x) if x == "" => " ",
        Some(x) => x,
        None => " "
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
        println!("details -> \t '{}' '{}' '{}' '{}' '{}' '{}'",
                 details, state, large_image, small_image, large_text, small_text);
    }
}


fn split_line(line: &str) -> (&str, &str) {
    let mut smaller_split_index = 0;
    let mut bigger_split_index = 0;
    for (i, j) in line.chars().enumerate() {
        if j == ' ' {
            if i >= 30 { // strings can't be longer than this
                break;
            }
            if i > 20 {
                bigger_split_index = i;
                break;
            } else if i > 4 { // strings can't be shorter than this
                smaller_split_index = i;
            }
        }
    }
    let split_index = match (smaller_split_index, bigger_split_index) {
        // does abs() exist in rust?
        (x, y) if (20 - x as i32) * (20 - x as i32) > (20 - y as i32) * (20 - y as i32) => y,
        (x, _) => x,
    };
    match (&line[0..split_index], &line[split_index + 1..]) {
        (x, y) if x.len() < 4 || y.len() < 4 || split_index == 0 => (line, "    "),
        (x, y) => (x, y)
    }
}


fn too_long(text: &str) -> &str {
    let mut output_length = 0;
    for i in text.split(" ") {
        if output_length + i.len() > 128 {
            break;
        } else {
            output_length += i.len() + 1; // to account for the split spaces
        }
    }
    &text[0..output_length]
}


pub fn main(mut client: &mut Client, commands: &HashMap<String, HashMap<String, String>>,
            parsed_input: Vec<&str>) {
    // the details and state values cannot be less than 4 or more than 29
    // the image texts cannot be longer than 128
    // all of these will change to "    " or "" to prevent crashes
    match parsed_input.get(0){
        Some(&"clear") => clear(&mut client),
        Some(x) => function(&mut client,
                            &parsed_input,
                            commands.get(*x)),
        _ => ()
    }
}
