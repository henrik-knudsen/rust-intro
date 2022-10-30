pub trait Animal {
    fn make_sound(&self) -> &'static str;
}

pub struct Duck;
pub struct Cat;

impl Animal for Duck {
    fn make_sound(&self) -> &'static str {
        "Quack"
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &'static str {
        "Meow"
    }
}

/// Repeat sound of animal three times.
/// Type of animal (T) is known at compile time. (Static dispatch)
fn repeat_sound_three_times<T: Animal>(animal: T) -> String {
    format!(
        "{} {} {}",
        animal.make_sound(),
        animal.make_sound(),
        animal.make_sound()
    )
}

/// Collects all the sounds of the animals in a Vec
/// Type of animals (&dyn Animal) are not known at compile time. (Dynamic dispatch)
fn get_all_sounds(animals: &Vec<&dyn Animal>) -> Vec<String> {
    animals.iter().map(|a| a.make_sound().to_owned()).collect()
}

/// Creates an animal based on sound given as argument, if it is a valid sound.
/// Look at boxes module for how to construct a Box of something.
fn construct_animal_by_sound(sound: &str) -> Option<Box<dyn Animal>> {
    match sound {
        "Quack" => Some(Box::new(Duck)),
        "Meow" => Some(Box::new(Cat)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_sound() {
        let duck = Duck;
        let cat = Cat;

        assert_eq!(duck.make_sound(), "Quack");
        assert_eq!(cat.make_sound(), "Meow");
    }

    #[test]
    fn test_repeat_sound_three_times() {
        let duck = Duck;
        let cat = Cat;

        assert_eq!(repeat_sound_three_times(duck), "Quack Quack Quack");
        assert_eq!(repeat_sound_three_times(cat), "Meow Meow Meow");
    }

    #[test]
    fn test_get_all_sounds() {
        let animals: Vec<&dyn Animal> = vec![&Duck, &Duck, &Cat, &Cat, &Duck];

        assert_eq!(
            get_all_sounds(&animals),
            vec!["Quack", "Quack", "Meow", "Meow", "Quack"]
        )
    }

    #[test]
    fn test_construct_animal_by_sound() {
        let quacking_animal = construct_animal_by_sound("Quack");
        let meowing_animal = construct_animal_by_sound("Meow");
        let silent_animal_maybe = construct_animal_by_sound("");

        assert!(quacking_animal.is_some());
        assert_eq!(quacking_animal.unwrap().make_sound(), "Quack");

        assert!(meowing_animal.is_some());
        assert_eq!(meowing_animal.unwrap().make_sound(), "Meow");

        assert!(silent_animal_maybe.is_none());
    }
}
