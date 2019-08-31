mod config;
mod context;
mod init;
mod module;
mod modules;
mod opt;
mod print;
mod segment;
mod utils;

use opt::{Opt, SubCommand};
use structopt::StructOpt;

use crate::module::ALL_MODULES;

fn main() {
    let opt = Opt::from_args();

    match &opt.sub_command {
        SubCommand::Init {
            print_full_init,
            shell,
        } => {
            if *print_full_init {
                init::init_main(shell).expect("can't init_main");
            } else {
                init::init_stub(shell).expect("can't init_stub");
            }
        }
        SubCommand::Prompt { common_opts: _ } => print::prompt(opt.clone()),
        SubCommand::Module {
            common_opts: _,
            list,
            shell,
        } => {
            if *list {
                println!("Supported modules list");
                println!("----------------------");
                for modules in ALL_MODULES {
                    println!("{}", modules);
                }
            }
            // TODO: maybe without clone
            print::module(shell, opt.clone());
        }
    };
}
