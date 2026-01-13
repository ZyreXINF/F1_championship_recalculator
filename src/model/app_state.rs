use crate::model::race_results::RaceResults;
use crate::model::settings::Settings;

#[derive(Debug)]
pub struct AppState {
    pub settings: Settings,
    pub race_results: RaceResults
}
impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {settings, race_results: RaceResults::new()}
    }
}