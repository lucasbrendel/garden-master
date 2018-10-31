use chrono::NaiveDate;
use plant::Plant;

struct Crop {
    pub num_plants: u32,
    pub date_planted: NaiveDate,
    pub plant: Plant,
}

impl Crop {
    fn new(plant: Plant, num_plants: u32, date_planted: NaiveDate) -> Self {
        Crop {
            plant,
            num_plants,
            date_planted,
        }
    }

    fn planned_harvest_date(&self) -> NaiveDate {
        self.date_planted + self.plant.days_to_maturity
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
