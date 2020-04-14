//! A command line interface to *Vehicle Routing Problem* solver.
//!

#[cfg(not(target_arch = "wasm32"))]
mod commands;

fn main() {
    cli::run_app()
}

#[cfg(not(target_arch = "wasm32"))]
mod cli {
    extern crate clap;
    use super::commands::import::{get_import_app, run_import};
    use super::commands::solve::{get_solve_app, run_solve};
    use clap::App;
    use std::process;

    pub fn run_app() {
        let matches = App::new("Vehicle Routing Problem Solver")
            .version("0.1")
            .author("Ilya Builuk <ilya.builuk@gmail.com>")
            .about("A command line interface to Vehicle Routing Problem solver")
            .subcommand(get_solve_app())
            .subcommand(get_import_app())
            .get_matches();

        match matches.subcommand() {
            ("solve", Some(solve_matches)) => run_solve(solve_matches),
            ("import", Some(import_matches)) => run_import(import_matches),
            ("", None) => {
                eprintln!("No subcommand was used. Use -h to print help information.");
                process::exit(1);
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod cli {
    pub fn run_app() {}
}
