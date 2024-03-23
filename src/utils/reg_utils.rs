use crate::utils::RegStruct;

use super::VecActions;

#[derive(Debug)]
pub enum RegTypes {
    Int(i64),
    Float(f32),
    BigFloat(f64),
}

impl RegStruct {
    pub fn new(data: &Vec<RegTypes>, name: &str, dependent: bool) -> Self {
        let mut vector: Vec<f32> = Vec::new();
        for i in 0..data.len() {
            match data[i] {
                RegTypes::Float(item) => vector.push(item),
                RegTypes::BigFloat(item) => vector.push(item as f32),
                RegTypes::Int(item) => vector.push(item as f32),
            }
        };
        let name = String::from(name);
        let length = vector.len() as f32;
        let mean = vector.mean_of_vec();
        let sum = vector.sum_of_vec();
        let mode = vector.vec_mode();
        let median = vector.vec_median();
        let variance = vector.vec_variance();
        let std_dev = vector.vec_std_dev();
        let squared_vector = vector.square_vec_values();
        let sum_of_squared = squared_vector.sum_of_vec();
        let min = vector.vec_min();
        let max = vector.vec_max();

        RegStruct {
            name,
            vector,
            squared_vector,
            dependent,
            length,
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

    pub fn is_dependent(&self) -> bool {
        self.dependent
    }

    pub fn summarize(&self) {
        println!("Name: {}", self.name);
        println!("Data {:?}", self.vector);
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