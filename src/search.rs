use crate::fact::Fact;

pub trait SearchAlgorithm {
    pub fn search(facts: Vec<Fact>);
}

pub struct BasicSearch {
    max_depth: usize,
}

impl SearchAlgorithm for BasicSearch {
    pub fn search(facts: Vec<Fact>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_stops_at_max_depth() {}
}
