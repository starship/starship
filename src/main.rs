#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let matches = App::new("Starship")
                    .about("The cross-platform prompt for astronauts. ✨🚀")
                    // pull the version number from Cargo.toml
                    .version(crate_version!())
                    // pull the authors from Cargo.toml
                    .author(crate_authors!())
                    .get_matches();

}
