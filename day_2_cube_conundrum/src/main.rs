use std::cmp::Ordering;

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.red.cmp(&other.red), self.green.cmp(&other.green), self.blue.cmp(&other.blue)) {
            (Less, Less, Less) => Some(Less),
            (Equal, Equal, Equal) => Some(Equal),
            (Greater, Greater, Greater) => Some(Greater)
        }
    }
}

fn main() {
    // iterate over game records
        // iterate over sets in a record
            // iterate over colors in a set
                // compare to max value the color has so far
        // identify whether the game was possible given the requirements
}
