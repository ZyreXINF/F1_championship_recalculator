use std::collections::BTreeMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PointSystem {
    pub gp_point_allocation: BTreeMap<u8, u16>,
    pub sprint_point_allocation: BTreeMap<u8, u16>,
    pub fastest_lap_point: u16,
    pub sprint_fastest_lap_point: u16,
    pub pole_point: u16,
    pub sprint_pole_point: u16
}

impl PointSystem {
    pub fn new() -> Self {
        Self {
            gp_point_allocation: Self::default_gp_points(),
            sprint_point_allocation: Self::default_sprint_points(),
            fastest_lap_point: 0,
            sprint_fastest_lap_point: 0,
            pole_point: 0,
            sprint_pole_point: 0
        }
    }

    fn default_gp_points() -> BTreeMap<u8, u16> {
        let mut map = BTreeMap::new();
        let points = [25, 18, 15, 12, 10, 8, 6, 4, 2, 1];
        for (pos, &pts) in (1..=10).zip(&points) {
            map.insert(pos, pts);
        }
        map
    }

    fn default_sprint_points() -> BTreeMap<u8, u16> {
        let mut map = BTreeMap::new();
        let points = [8, 7, 6, 5, 4, 3, 2, 1];
        for (pos, &pts) in (1..=8).zip(&points) {
            map.insert(pos, pts);
        }
        map
    }

    pub fn view_point_system(&self) {
        for (pos, pts) in &self.gp_point_allocation {
            println!("P{}: {} points", pos, pts);
        }

        for (pos, pts) in &self.sprint_point_allocation {
            println!("Sprint P{}: {} points", pos, pts);
        }

        println!("Fastest Lap points: {} points, Sprint Fastest Lap points: {}\n\
         Pole points: {} points, Sprint Pole points: {}",
                 self.fastest_lap_point, self.sprint_fastest_lap_point, self.pole_point, self.sprint_pole_point);
    }
}