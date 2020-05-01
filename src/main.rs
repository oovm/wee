pub mod cfgs;
pub mod error;
pub use cfgs::get_cmds;

use clap::{crate_authors, crate_description, crate_name, crate_version, App};

fn main() {
    let app = App::new(crate_name!())
        .version(&crate_version!()[..])
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .arg("<INPUT>             'Sets the input file to use'")
        .arg("-t...               'Sets the level of verbosity'")
        .arg("-s                  'Show all available scripts'")
        .get_matches();
}
