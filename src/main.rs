#[macro_use(value_t)]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("hb-master")
        .version("0.1.0")
        .about("HellBender Master Node")
        .author("Sebastian Woetzel")
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .help("The port nodes connect to. (Default: 6268)")
             .takes_value(true)
             .value_name("PORT"))
        .get_matches();

    let port = value_t!(matches.value_of("port"), u16).unwrap_or(6862);
    println!("Listening on port {}", port);
}
