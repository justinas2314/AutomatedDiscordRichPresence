mod parser;
mod get_apps;
mod client;

extern crate discord_rpc_client;

use discord_rpc_client::Client;


fn main() {
    let ini_contents= std::fs::read_to_string(
        "config\\config.ini").unwrap();
    // my hard-coded client change this to use a different client
    let mut rpc_client = Client::new(696035653711953981)
        .unwrap();
    rpc_client.start();
    let (dict_commands, order) = parser::main(&ini_contents);
    loop {
        // updates every minute
        let running_apps = get_apps::main(&order);
        let running_app = match parser::get_ordered(running_apps, &order) {
            Some(x) => x,
            None => ("clear".to_string(), "clear".to_string())
        };
        println!("this app detected -> '{}'\nthis title detected -> '{}'",
                 &running_app.1, &running_app.0);
        // &running_app.1 must be the first arg in parsed_input
        // the var parsed_input is not the actual parsed input
        // back in my day this used to work differently
        client::main(&mut rpc_client, &dict_commands,
                     vec![&running_app.1, &running_app.0]);
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
