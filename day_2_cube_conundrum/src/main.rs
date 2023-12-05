use std::{cmp::Ordering, collections::HashSet};

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize
}
impl PartialEq for CubeSet {
    fn eq(&self, other: &Self) -> bool {
        return self.red == other.red && self.green == other.green && self.blue == other.blue;
    }
}
impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let differences: HashSet<Ordering> = HashSet::from(
            [self.red.cmp(&other.red), self.green.cmp(&other.green), self.blue.cmp(&other.blue)]);
        if differences.contains(&Ordering::Greater) && differences.contains(&Ordering::Less) {
            return None
        }
        else if differences.contains(&Ordering::Greater) {
            return Some(Ordering::Greater);
        }
        else if differences.contains(&Ordering::Less) {
            return Some(Ordering::Less);
        }
        else {
            return Some(Ordering::Equal);
        }
    }
}

fn pt_1() -> usize {
    0
}

fn main() {
    // iterate over game records
        // iterate over sets in a record
            // iterate over colors in a set
                // compare to max value the color has so far
        // identify whether the game was possible given the requirements
}
