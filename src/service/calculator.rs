use crate::model::driver::{ChampionshipPosition, Driver};
use crate::model::point_system::PointSystem;

pub fn recalculate_championship(point_system: &PointSystem, driver_results: Vec<Driver>) -> Vec<ChampionshipPosition>{
    let mut standings: Vec<ChampionshipPosition> = Vec::new();
    for driver in driver_results {
        let mut points_scored: u32 = 0;
        for finish in &driver.finish_history {
            points_scored += *point_system.gp_point_allocation.get(finish).unwrap_or(&0) as u32;
        }
        for sprint_finish in &driver.sprint_finish_history {
            points_scored += *point_system.sprint_point_allocation.get(sprint_finish).unwrap_or(&0) as u32
        }
        points_scored += (driver.fastest_laps as u32) * (point_system.fastest_lap_point as u32);
        points_scored += (driver.sprint_fastest_laps as u32) * (point_system.sprint_fastest_lap_point as u32);
        points_scored += (driver.poles as u32) * (point_system.pole_point as u32);
        points_scored += (driver.poles as u32) * (point_system.sprint_pole_point as u32);
        standings.push(ChampionshipPosition::new(driver.full_name, points_scored, driver.wins, driver.sprint_wins, driver.fastest_laps, driver.sprint_fastest_laps, driver.poles, driver.sprint_poles));
    }
    standings.sort_by(|a, b| {
        b.points_scored.cmp(&a.points_scored)
            .then_with(|| b.wins.cmp(&a.wins))
            .then_with(|| b.fastest_laps.cmp(&a.fastest_laps))
    });
    // standings.sort_by(|a, b| b.points_scored.cmp(&a.points_scored));
    standings
}