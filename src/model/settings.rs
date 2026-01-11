use crate::model::point_system::PointSystem;

#[derive(Debug)]
pub struct Settings{
    pub championship_year: u16,
    pub point_system: PointSystem
}
impl Settings{
    pub fn new(championship_year: u16, point_system: PointSystem) -> Self {
        Self { championship_year, point_system }
    }

    pub fn view_settings(&self){
        println!("\nChampionship Year: {}" , self.championship_year);
        self.point_system.view_point_system();
    }
}