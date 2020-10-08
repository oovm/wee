mod store;

pub use self::store::Store;

// use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use colored::*;
use std::time::Instant;
use subprocess::Exec;

use clap::Parser;

/// Should do early return
pub type ShouldReturn = bool;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Wee {
    /// Sets the input file to use
    cmd: String,
    /// Show all available scripts
    #[clap(short, long)]
    show: usize,
    /// Show execution time
    #[clap(short, long)]
    time: usize,
    #[clap(short, long)]
    dump: String,
    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let app = Wee::parse();
    let store = Store::read_configs();
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    if app.show != 0 {
        store.print_scripts(app.show);
        return;
    }

    let now = Instant::now();
    match store.get_script(&app.cmd) {
        None => println!("{}", format!("Command: '{}' not found!", app.cmd).red()),
        Some(s) => {
            if let Ok(_) = Exec::shell(s).join() {
                match app.time {
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
    }
    // if let Ok(_) = Exec::shell("wee --help").join() {
    //     return;
    // }
}

#[test]
fn test() {}
