#[derive(Debug)]
pub struct Driver{
    pub finish_history: Vec<u8>,
    pub full_name: String,
    pub fastest_laps: u8,
}

#[derive(Debug)]
pub struct ChampionshipPosition{
    pub full_name: String,
    pub points_scored: u32
}
impl ChampionshipPosition {
    pub fn new(full_name: String, points_scored: u32) -> Self {
        Self { full_name, points_scored }
    }
}