//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.

use chrono::{Duration};

/// Standard type to define all things to grow
struct Plant {
    /// Name of a plant.
    name: String,
    /// Seasonal type of plant
    plant_type: PlantType,
    /// Growing zones defined by USDA that a plant can survive in. This is limited to first 10 zones.
    zones: [u8; 10],
    /// Any description or textual things to track about the plant.
    notes: String,
    /// Number of days from planting until germination occurs
    days_to_germinate: Duration,
    /// Number of days from planting until harvesting is ready
    days_to_harvest: Duration
}

/// Seasonal variety types of plants
#[derive(Debug, PartialEq)]
enum PlantType {
    /// Plant has one growing season and needs to be replanted every year
    Annual,
    /// Plant survives over multiple growing seasons.
    Perennial,
}

impl Plant {
    /// Create a new plant type
    ///
    /// # Examples
    /// ```
    /// let plant = Plant::new("Tomato");
    /// assert_eq!("Tomato", plant.name);
    /// ```
    fn new(name: String, germinate: i64, harvest: i64) -> Self {
        Plant {
            name,
            plant_type: PlantType::Annual,
            notes: String::from(""),
            zones: [0,0,0,0,0,0,0,0,0,0],
            days_to_harvest: Duration::days(harvest),
            days_to_germinate: Duration::days(germinate)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name() {
        let plant = Plant::new(String::from("Tomato"), 30, 50);
        assert_eq!("Tomato", plant.name);
    }

    #[test]
    fn default_plant_type() {
        let plant = Plant::new(String::from("Potato"), 50, 70);
        assert_eq!(PlantType::Annual, plant.plant_type);
    }
}
