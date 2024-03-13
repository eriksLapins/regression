use crate::utils::{RegStruct, RegUtils};

use super::VecActions;

impl RegUtils<i64> for RegStruct<i64> {
    fn new(data: &Vec<i64>, dependent: bool) -> Self {
        let vector = Vec::from(data.clone());
        let length = vector.len();
        let mean = vector.mean_of_vec();
        let sum = vector.sum_of_vec();
        let mode = vector.vec_mode();
        let median = vector.vec_median();
        let variance = vector.vec_variance();
        let std_dev = vector.vec_std_dev();
        let squared_vector = vector.square_vec_values();
        let sum_of_squared = squared_vector.sum_of_vec();
        let min = vector.iter().min().unwrap().clone();
        let max = vector.iter().max().unwrap().clone();
        RegStruct {
            vector,
            squared_vector,
            dependent,
            length: length as i64,
            mean,
            median,
            std_dev,
            mode,
            variance,
            sum,
            sum_of_squared,
            min,
            max
        }
    }
    fn is_dependent(&self) -> bool {
        self.dependent
    }
    fn summarize(&self) {
        println!("Data i64 {:?}", self.vector);
        println!("Mean: {}", self.mean);
        println!("Median: {}", self.median);
        println!("Mode: {}", self.mode);
        println!("Standard deviation: {}", self.std_dev);
        println!("Variance: {}", self.variance);
        println!("Sum: {}", self.sum);
        println!("Mean: {}", self.mean);
        println!("Min: {}", self.min);
        println!("Max: {}", self.max);
    }
}

impl RegUtils<f32> for RegStruct<f32> {
    fn new(data: &Vec<f32>, dependent: bool) -> Self {
        let vector = Vec::from(data.clone());
        let length = vector.len();
        let mean = vector.mean_of_vec();
        let sum = vector.sum_of_vec();
        let mode = vector.vec_mode();
        let median = vector.vec_median();
        let variance = vector.vec_variance();
        let std_dev = vector.vec_std_dev();
        let squared_vector = vector.square_vec_values();
        let sum_of_squared = squared_vector.sum_of_vec();
        let min = vector.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone();
        let max = vector.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone();
        RegStruct {
            vector,
            squared_vector,
            dependent,
            length: length as f32,
            mean,
            median,
            std_dev,
            mode,
            variance,
            sum,
            sum_of_squared,
            min,
            max
        }
    }
    fn is_dependent(&self) -> bool {
        self.dependent
    }
    fn summarize(&self) {
        println!("Data f32 {:?}", self.vector);
        println!("Mean: {}", self.mean);
        println!("Median: {}", self.median);
        println!("Mode: {}", self.mode);
        println!("Standard deviation: {}", self.std_dev);
        println!("Variance: {}", self.variance);
        println!("Sum: {}", self.sum);
        println!("Mean: {}", self.mean);
        println!("Min: {}", self.min);
        println!("Max: {}", self.max);
    }
}