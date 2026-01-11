pub mod model;
pub mod championship;
pub mod api;
pub mod io;

use io::cli::init_cli;
use crate::model::app_state::AppState;

pub fn run(state: &mut AppState) {
    init_cli(state);
}
