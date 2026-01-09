use crate::points::point_system::Point_system;

pub struct Settings{
    championship_year: u16,
    point_system: Point_system
}
impl Settings{
    pub fn new(championship_year: u16, point_system: Point_system) -> Self {
        Self { championship_year, point_system }
    }
}