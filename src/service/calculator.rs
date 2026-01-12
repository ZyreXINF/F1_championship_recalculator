use crate::model::driver::{ChampionshipPosition, Driver};
use crate::model::point_system::PointSystem;

pub fn recalculate_championship(point_system: &PointSystem, driver_results: Vec<Driver>) -> Vec<ChampionshipPosition>{
    let mut standings: Vec<ChampionshipPosition> = Vec::new();
    for driver in driver_results {
        let mut points_scored: u32 = 0;
        for finish in &driver.finish_history {
            points_scored += (*point_system.gp_point_allocation.get(finish).unwrap_or(&0) as u32);
        }
        standings.push(ChampionshipPosition::new(driver.full_name, points_scored));
    }
    standings.sort_by(|a, b| b.points_scored.cmp(&a.points_scored));
    standings
}