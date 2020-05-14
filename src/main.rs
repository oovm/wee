pub mod cfgs;
pub mod error;

pub use cfgs::Store;

use colored::*;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

fn main() {
    let app = App::new(crate_name!())
        .version(&crate_version!()[..])
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("show")
                .short('s')
                .long("show")
                .about("Show all available scripts"),
        )
        .arg(
            Arg::with_name("cmd")
                .required(false)
                .index(1)
                .about("Sets the input file to use"),
        )
        .get_matches();

    let store = Store::read_configs();

    match app.occurrences_of("show") {
        0 => {}
        _ => {
            println!("{}", format!("All available commands: {}", store.scripts.len()).purple());
            for (k, v) in store.scripts {
                if v.trim().lines().count() == 1 {
                    print!("{}: \"{}\"", k.green(), v)
                } else {
                    println!("{}: \"\"\"\n{}\"\"\"", k.green(), v)
                }
            }
            return;
        }
    }

    match app.value_of("cmd") {
        Some(o) => match store.scripts.get(o) {
            None => println!("Command: '{}' not found!", o),
            Some(s) => println!("{}", s),
        },
        None => println!("wee --help"),
    }
}
