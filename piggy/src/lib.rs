use std::time::{SystemTime, UNIX_EPOCH};

/// Derive macro that implements the [`Pig`] trait for a struct.
///
/// The pig is picked deterministically — same struct name on the same day gets the same pig.
///
/// ```
/// use piggy::Pig;
///
/// #[derive(Pig)]
/// struct Dog;
///
/// let s = Dog;
/// assert!(s.pig().starts_with("Dog today is a"));
/// ```
pub use piggy_derive::Pig;

/// A struct whose instances can report which pig they are today.
pub trait Pig {
    /// Returns a string like `"MyStruct today is a Watermelon Pig"`.
    fn pig(&self) -> String;
}

const PIG_NAMES: &[&str] = &[
    "Watermelon Pig",
    "Pig Dice",
    "Toy Pig",
    "Snow pig",
    "Apple Pig",
    "Humans",
    "Piggy Meow",
    "Rainbow Pig",
    "Pork Chops",
    "Paradise Pig",
    "Hell Pig",
    "Demon Pig",
    "Mechanical Pig",
    "Android Pig",
    "Old Pig Calendar",
    "Soul Pig",
    "Tearful Pig",
    "Pig Turtle",
    "Pigman",
    "Black Pig Sesame Dumplings",
    "The Bound Pig King",
    "Zombie Pig",
    "Today is not for pigs:<"
];

/// Deterministically picks a pig name from a curated list, based on `struct_name` and today's date.
/// The same input on the same day always returns the same pig.
pub fn pick_pig_for(struct_name: &str) -> &'static str {
    /* We don't need professional random number. */
    let days = days_since_epoch();
    let name_hash = stupid_hash(struct_name);
    let mut seed = (days ^ name_hash) as u32;
    seed = seed.wrapping_mul(114514).wrapping_add(1919810);
    let idx = (seed as usize) % PIG_NAMES.len();
    PIG_NAMES[idx]
}

fn days_since_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() / 86400
}

fn stupid_hash(s: &str) -> u64 {
    let mut hash = 0u64;
    for b in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u64);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pick_pig() {
        let pig = pick_pig_for("Dog");
        assert!(PIG_NAMES.contains(&pig));
        assert_eq!(pick_pig_for("Dog"), pig);
    }
}
