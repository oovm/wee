pub mod cfgs;
pub mod error;

pub use cfgs::Store;

// use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use colored::*;
use std::time::Instant;
use subprocess::Exec;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Wee {
    /// Sets the input file to use

    cmd: String,
    /// Show all available scripts
    #[clap(short, long)]
    show: String,
    /// Show execution time
    #[clap(short, long)]
    time: String,
    #[clap(short, long)]
    dump: String,
    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}



fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("show")
                .short('s')
                .long("show")
                .about("Show all available scripts"),
        )
        .arg(
            Arg::with_name("dump")
                .short('d')
                .long("dump")
                .about("Use dump to avoid repeatedly scanning configuration files"),
        )
        .arg(
            Arg::with_name("time")
                .short('t')
                .long("time")
                .about("Show execution time"),
        )
        .arg(
            Arg::with_name("cmd")
                .required(false)
                .index(1)
                .about("Sets the input file to use"),
        )
        .get_matches();

    let store = Store::read_configs();
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    match app.occurrences_of("show") {
        0 => {}
        _ => {
            println!(
                "{}",
                format!("All available commands: {}", store.scripts.len()).purple()
            );
            for (k, v) in store.scripts {
                if v.trim().lines().count() == 1 {
                    println!("{}: \"{}\"", k.green(), v)
                } else {
                    println!("{}: \"\"\"\n{}\"\"\"", k.green(), v)
                }
            }
            return;
        }
    }

    let now = Instant::now();
    match app.value_of("cmd") {
        Some(o) => match store.scripts.get(o) {
            None => println!("{}", format!("Command: '{}' not found!", o).red()),
            Some(s) => {
                if let Ok(_) = Exec::shell(s).join() {
                    match app.occurrences_of("time") {
                        0 => {
                            return;
                        }
                        _ => println!(
                            "{}",
                            format!("finished in {}s", now.elapsed().as_secs_f64()).blue()
                        ),
                    };
                }
            }
        },
        None => {
            if let Ok(_) = Exec::shell("wee --help").join() {
                return;
            }
        }
    }
}
