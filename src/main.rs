mod commands;

use commands::download;
use commands::list;
use seahorse::{App, Context};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage(format!("{} [cmd] [args]\n", env!("CARGO_PKG_NAME")))
        .action(|c: &Context| c.help())
        .command(list::command())
        .command(download::command());

    app.run(args);
}
