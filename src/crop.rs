use chrono::{Duration, NaiveDate};
use plant::Plant;

/// Tracking the growth and harvest of a specific plant
pub struct Crop {
    /// Number of plants sown
    pub num_plants: u32,
    /// The date that the plants were planted
    pub date_planted: NaiveDate,
    /// The type of plant sown
    pub plant: Plant,
}

impl Crop {
    /// Create a new crop instance
    fn new(plant: Plant, num_plants: u32, date_planted: NaiveDate) -> Self {
        Crop {
            plant,
            num_plants,
            date_planted,
        }
    }

    /// Provides the ideal harvest date based on planting date and time to maturity
    fn planned_harvest_date(&self) -> NaiveDate {
        self.date_planted + Duration::days(self.plant.days_to_maturity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use plant::PlantType;

    #[test]
    fn new_crop() {
        let plant = Plant::new(String::from("Tomato"), 50, PlantType::Annual);
        let crop = Crop::new(plant, 5, NaiveDate::from_ymd(2018, 5, 6));
        assert_eq!(5, crop.num_plants);
    }

    #[test]
    fn harvest_date() {
        let plant = Plant::new(String::from("Tomato"), 50, PlantType::Annual);
        let crop = Crop::new(plant, 5, NaiveDate::from_ymd(2018, 5, 6));
        assert_eq!(
            NaiveDate::from_ymd(2018, 6, 25),
            crop.planned_harvest_date()
        );
    }
}
