pub mod model;
pub mod points;
pub mod championship;
pub mod api;
pub mod io;

use io::cli::init_cli;

pub fn run() {
    init_cli();
}
