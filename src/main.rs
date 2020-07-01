mod get_apps;
mod read_commands;
mod client;

extern crate discord_rpc_client;

use discord_rpc_client::Client;


fn main() {
    // should switch to using .ini files instead of commands.txt
    // doesn't work on linux yet
    let contents= std::fs::read_to_string(
        "config\\commands.txt").unwrap();
    let mut rpc_client = Client::new(696035653711953981)
        .unwrap();
    rpc_client.start();
    let commands = read_commands::main(&contents);
    let order = read_commands::order(&contents);
    let bases = read_commands::bases(&contents);
    loop {
        // no sleeping because this part is really slow (thanks windows)
        let running_apps = get_apps::main(&commands);
        let running_app = match read_commands::get_ordered(running_apps, &order) {
            Some(x) => x,
            None => ("clear".to_string(), "clear".to_string())
        };
        println!("this app detected -> '{}'\nthis title detected -> '{}'", &running_app.1, &running_app.0);
        // &running_app.1 must be the first arg in parsed_input
        client::main(&mut rpc_client, &commands, &bases,
                     vec![&running_app.1, &running_app.0, &running_app.0, &running_app.0, &running_app.0]);
    }
}
