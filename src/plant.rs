//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.

use chrono::{Duration};

struct Plant {
    name: String,
    plant_type: PlantType,
    zones: [u8; 10],
    notes: String,
    days_to_germinate: Duration,
    days_to_harvest: Duration
}

#[derive(Debug, PartialEq)]
enum PlantType {
    Annual,
    Perennial,
}

impl Plant {
    fn new(name: String) -> Self {
        Plant {
            name,
            plant_type: PlantType::Annual,
            notes: String::from(""),
            zones: [0,0,0,0,0,0,0,0,0,0],
            days_to_harvest: Duration::days(50),
            days_to_germinate: Duration::days(50)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name() {
        let plant = Plant::new(String::from("Tomato"));
        assert_eq!("Tomato", plant.name);
    }

    #[test]
    fn default_plant_type() {
        let plant = Plant::new(String::from("Potato"));
        assert_eq!(PlantType::Annual, plant.plant_type);
    }
}
