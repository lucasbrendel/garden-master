//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.
struct Plant {
    name: String,
    plant_type: PlantType,
}

enum PlantType {
    Annual,
    Perennial,
}

impl Plant {
    fn new(name: String) -> Plant {
        Plant {
            name,
            plant_type: PlantType::Annual,
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
}
