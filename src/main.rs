use f1_championship_recalculator::model::app_state::AppState;
use f1_championship_recalculator::model::settings::Settings;
use f1_championship_recalculator::model::point_system::PointSystem;

fn main() {
    let default_settings = Settings::new(2025, PointSystem::new());
    f1_championship_recalculator::run(&mut AppState::new(default_settings));
}
