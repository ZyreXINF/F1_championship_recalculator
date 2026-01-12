use crate::model::settings::Settings;

#[derive(Debug)]
pub struct AppState {
    pub settings: Settings,
}
impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self { settings}
    }
}