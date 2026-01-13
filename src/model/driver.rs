#[derive(Debug)]
#[derive(Clone)]
pub struct Driver{
    pub finish_history: Vec<u8>,
    pub sprint_finish_history: Vec<u8>,
    pub full_name: String,
    pub wins: u8,
    pub sprint_wins: u8,
    pub fastest_laps: u8,
    pub sprint_fastest_laps: u8,
    pub poles: u8,
    pub sprint_poles: u8
}

#[derive(Debug)]
pub struct ChampionshipPosition{
    pub full_name: String,
    pub points_scored: u32,
    pub wins: u8,
    pub sprint_wins: u8,
    pub fastest_laps: u8,
    pub sprint_fastest_laps: u8,
    pub poles: u8,
    pub sprint_poles: u8
}
impl ChampionshipPosition {
    pub fn new(full_name: String, points_scored: u32, wins: u8, sprint_wins: u8, fastest_laps: u8, sprint_fastest_laps: u8, poles: u8, sprint_poles: u8) -> Self {
        Self { full_name, points_scored, wins, sprint_wins, fastest_laps, sprint_fastest_laps, poles, sprint_poles }
    }
}