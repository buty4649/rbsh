use reddish::App;
use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    exit(App::run(args))
}
