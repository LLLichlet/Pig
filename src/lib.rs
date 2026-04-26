use piggy::Pig;

#[derive(Pig)]
struct Dog;

#[derive(Pig)]
struct Cat;

#[derive(Pig)]
struct Human;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dog_has_pig() {
        assert!(Dog.pig().contains("Dog today is a"));
    }

    #[test]
    fn cat_has_pig() {
        assert!(Cat.pig().contains("Cat today is a"));
    }

    #[test]
    fn human_has_pig() {
        assert!(Human.pig().contains("Human today is a"));
    }

    #[test]
    fn same_struct_same_pig_in_one_day() {
        assert_eq!(Dog.pig(), Dog.pig());
        assert_eq!(Cat.pig(), Cat.pig());
    }
}
