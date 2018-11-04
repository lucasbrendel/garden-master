//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.

use strum::AsStaticRef;

/// Standard type to define all things to grow
#[derive(Debug)]
pub struct Plant {
    /// Name of a plant.
    pub name: String,
    /// Seasonal type of plant
    pub plant_type: PlantType,
    /// Growing zones defined by USDA that a plant can survive in. This is limited to first 10 zones.
    pub zones: Vec<u8>,
    /// Any description or textual things to track about the plant.
    pub notes: String,
    /// Number of days from planting until germination occurs
    pub days_to_maturity: i64,
}

/// Seasonal variety types of plants
#[derive(Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
pub enum PlantType {
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
    pub fn new(name: String, maturity: i64, season_type: PlantType) -> Self {
        Plant {
            name,
            plant_type: season_type,
            notes: String::from(""),
            zones: Vec::new(),
            days_to_maturity: maturity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name() {
        let plant = Plant::new(String::from("Tomato"), 30, PlantType::Annual);
        assert_eq!("Tomato", plant.name);
        assert_eq!(30, plant.days_to_maturity);
    }

    #[test]
    fn default_plant_type() {
        let plant = Plant::new(String::from("Potato"), 50, PlantType::Annual);
        assert_eq!(PlantType::Annual, plant.plant_type);
    }
}
