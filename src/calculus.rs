use std::cell::Cell;
use std::collections::HashMap;

pub struct Stats {
    numbers: Vec<i32>,
    calculated: Cell<bool>,
    mean: Cell<f64>,
    median: Cell<i32>,
    mode: Cell<i32>,
}

impl Stats {
    pub fn create(numbers: Vec<i32>) -> Stats {
        Stats {
            numbers,
            calculated: Cell::from(false),
            mean: Cell::from(0.0),
            median: Cell::from(0),
            mode: Cell::from(0),
        }
    }

    pub fn show_results(&self) {
        if !self.calculated.get() {
            self.mean();
            self.median();
            self.mode();

            self.calculated.set(true);
        }

        println!("{}", self.mean.get());
        println!("{}", self.median.get());
        println!("{}", self.mode.get());
    }

    fn mean(&self) {
        self.mean
            .set(self.numbers.iter().sum::<i32>() as f64 / self.numbers.len() as f64);
    }

    fn median(&self) {
        let mut new_vector = self.numbers.to_vec();
        new_vector.sort();
        self.median.set(
            new_vector
                .get(new_vector.len() / 2)
                .cloned()
                .expect("Invalid vector"),
        );
    }

    fn mode(&self) {
        let mut occurrences = HashMap::new();
        for &number in &self.numbers {
            *occurrences.entry(number).or_insert(0) += 1;
        }

        self.mode.set(
            occurrences
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(val, _)| val)
                .expect("Cannot compute the mode of zero numbers"),
        )
    }
}
