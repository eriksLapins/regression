
use crate::utils::*;
use crate::traits::VecActions;
use std::collections::HashMap;

use self::reg_utils::RegTypes;

impl VecActions<f32> for Vec<f32> {
    fn sum_of_vec(&self) -> f32 {
        let number = self.clone().into_iter().map(|v| v).sum::<f32>();
        number as f32
    }
    fn mean_of_vec(&self) -> f32 {
        let sum = self.sum_of_vec();
        let count: f32 = self.len() as f32;
        let mean = sum / count;
        mean
    }
    fn square_vec_values(&self) -> Vec<f32> {
        let cloned = self.clone();
        let numbers = cloned.iter().map(|number| number.powf(2.0) as f32);
        numbers.collect_vec()   
    }
    fn vec_merge(&self, vec_2: &Vec<f32>, action: MergeActions) -> Vec<f32> {
        let new_array = self.clone();
        if new_array.len() != vec_2.len() {
            panic!("The arrays are of different lengths")
        }

        let mut return_array = Vec::new();
        for i in 0..new_array.len() {
            match action {
                MergeActions::Multiply => return_array.push(new_array[i] * vec_2[i]),
                MergeActions::Divide => return_array.push(new_array[i] / vec_2[i]), 
                MergeActions::Sum => return_array.push(new_array[i] + vec_2[i]), 
                MergeActions::Subtract => return_array.push(new_array[i] - vec_2[i]), 
                MergeActions::Power => return_array.push(new_array[i].powf(vec_2[i].try_into().expect("Second vectors values have to be U32 compatible"))), 
            }
        }

        return_array
    }
    fn vec_median(&self) -> f32 {
        let mut clone = self.clone();
        clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = clone.len() / 2;
        let median = clone[mid];
        median
    }

    fn vec_mode(&self) -> f32 {
        let mut times = HashMap::new();

      // count
      for x in self {
          let cnt = times.entry(*x as usize).or_insert(0.0);
          *cnt += 1.0;
      }

      let mut best: (f32, f32) = (*times.iter().nth(0).expect("Fatal.").0 as f32, *times.iter().nth(0).expect("Fatal.").1 as f32);

      for x in times.iter() {
          if *x.1 > best.1 {
              best = (*x.0 as f32, *x.1);
          }
      }
      best.0
    }
    fn vec_variance(&self) -> f32 {
        let clone = self.clone();
        let mean = clone.mean_of_vec();
        let numerator: f32 = clone.into_iter().map(|num| (num - mean).powf(2.0)).sum();
        numerator / self.len() as f32
    }
    fn vec_std_dev(&self) -> f32 {
        let clone = self.clone();
        clone.vec_variance().sqrt()
    }
    fn to_regtypes(&self, _method: VecFillMethod) -> Vec<reg_utils::RegTypes> {
        self.into_iter().map(|x| RegTypes::Float(*x)).collect_vec()
    }
    fn vec_max(&self) -> f32 {
        self.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
    }
    fn vec_min(&self) -> f32 {
        self.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
    }
}

impl VecActions<f64> for Vec<f64> {
    fn sum_of_vec(&self) -> f64 {
        let number = self.clone().into_iter().map(|v| v).sum::<f64>();
        number as f64
    }
    fn mean_of_vec(&self) -> f32 {
        let sum = self.sum_of_vec() as f32;
        let count: f32 = self.len() as f32;
        let mean = sum / count;
        mean
    }
    fn square_vec_values(&self) -> Vec<f64> {
        let cloned = self.clone();
        let numbers = cloned.iter().map(|number| number.powf(2.0) as f64);
        numbers.collect_vec()   
    }
    fn vec_merge(&self, vec_2: &Vec<f64>, action: MergeActions) -> Vec<f64> {
        let new_array = self.clone();
        if new_array.len() != vec_2.len() {
            panic!("The arrays are of different lengths")
        }

        let mut return_array = Vec::new();
        for i in 0..new_array.len() {
            match action {
                MergeActions::Multiply => return_array.push(new_array[i] * vec_2[i]),
                MergeActions::Divide => return_array.push(new_array[i] / vec_2[i]), 
                MergeActions::Sum => return_array.push(new_array[i] + vec_2[i]), 
                MergeActions::Subtract => return_array.push(new_array[i] - vec_2[i]), 
                MergeActions::Power => return_array.push(new_array[i].powf(vec_2[i].try_into().expect("Second vectors values have to be U32 compatible"))), 
            }
        }

        return_array
    }
    fn vec_median(&self) -> f64 {
        let mut clone = self.clone();
        clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = clone.len() / 2;
        let median = clone[mid];
        median
    }

    fn vec_mode(&self) -> f64 {
        let mut times = HashMap::new();

      // count
      for x in self {
          let cnt = times.entry(*x as usize).or_insert(0.0);
          *cnt += 1.0;
      }

      let mut best: (f64, f64) = (*times.iter().nth(0).expect("Fatal.").0 as f64, *times.iter().nth(0).expect("Fatal.").1 as f64);

      for x in times.iter() {
          if *x.1 > best.1 {
              best = (*x.0 as f64, *x.1);
          }
      }
      best.0
    }
    fn vec_variance(&self) -> f32 {
        let clone = self.clone();
        let mean = clone.mean_of_vec() as f32;
        let numerator: f32 = clone.into_iter().map(|num| ((num as f32) - mean).powf(2.0)).sum();
        numerator / self.len() as f32
    }
    fn vec_std_dev(&self) -> f32 {
        let clone = self.clone();
        clone.vec_variance().sqrt()
    }
    fn to_regtypes(&self, _method: VecFillMethod) -> Vec<reg_utils::RegTypes> {
        self.into_iter().map(|x| RegTypes::BigFloat(*x)).collect_vec()
    }
    fn vec_max(&self) -> f64 {
        self.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
    }
    fn vec_min(&self) -> f64 {
        self.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
    }
}

impl VecActions<i64> for Vec<i64> {
    fn sum_of_vec(&self) -> i64 {
        let number = self.clone().into_iter().map(|v| v).sum::<i64>();
        number
    }
    fn mean_of_vec(&self) -> f32 {
        let sum = self.sum_of_vec() as f32;
        let count: f32 = self.len() as f32;
        let mean = sum / count;
        mean
    }
    fn square_vec_values(&self) -> Vec<i64> {
        let cloned = self.clone();
        let numbers = cloned.iter().map(|number| number.pow(2));
        numbers.collect_vec()   
    }
    fn vec_merge(&self, vec_2: &Vec<i64>, action: MergeActions) -> Vec<i64> {
        let new_array = self.clone();
        if new_array.len() != vec_2.len() {
            panic!("The arrays are of different lengths")
        }

        let mut return_array = Vec::new();
        for i in 0..new_array.len() {
            match action {
                MergeActions::Multiply => return_array.push(new_array[i] * vec_2[i]),
                MergeActions::Divide => return_array.push(new_array[i] / vec_2[i]), 
                MergeActions::Sum => return_array.push(new_array[i] + vec_2[i]), 
                MergeActions::Subtract => return_array.push(new_array[i] - vec_2[i]), 
                MergeActions::Power => return_array.push(new_array[i].pow(vec_2[i] as u32)), 
            }
        }

        return_array
    }
    fn vec_median(&self) -> i64 {
        let mut clone = self.clone();
        clone.sort();
        let mid = clone.len() / 2;
        let median = clone[mid];
        median
    }

    fn vec_mode(&self) -> i64 {
        let mut times = HashMap::new();

      // count
      for x in self {
          let cnt = times.entry(*x as usize).or_insert(0);
          *cnt += 1;
      }

      let mut best: (i64, i64) = (*times.iter().nth(0).expect("Fatal.").0 as i64, *times.iter().nth(0).expect("Fatal.").1 as i64);

      for x in times.iter() {
          if *x.1 > best.1 {
              best = (*x.0 as i64, *x.1);
          }
      }
      best.0
    }
    fn vec_variance(&self) -> f32 {
        let clone = self.clone();
        let mean = clone.mean_of_vec();
        let numerator: f32 = clone.into_iter().map(|num| (num as f32 - mean).powf(2.0)).sum();
        numerator / self.len() as f32
    }
    fn vec_std_dev(&self) -> f32 {
        let clone = self.clone();
        clone.vec_variance().sqrt()
    }
    
    fn to_regtypes(&self, _method: VecFillMethod) -> Vec<reg_utils::RegTypes> {
        self.into_iter().map(|x| RegTypes::Int(*x)).collect_vec()
    }
    fn vec_max(&self) -> i64 {
        self.iter().max().unwrap().clone()
    }
    fn vec_min(&self) -> i64 {
        self.iter().min().unwrap().clone()
    }
}

impl VecActions<f32> for Vec<Option<f32>> {
    fn sum_of_vec(&self) -> f32 {
        let mut clean_vec: Vec<f32> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        } 
        let number = clean_vec.into_iter().map(|v| v).sum::<f32>();
        number as f32
    }
    fn mean_of_vec(&self) -> f32 {
        let mut clean_vec: Vec<f32> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        } 
        let sum = self.sum_of_vec();
        let count: f32 = clean_vec.len() as f32;
        let mean = sum / count;
        mean
    }
    fn square_vec_values(&self) -> Vec<f32> {
        panic!("Squaring vector values is only supported for Vec<T> not Vec<Option<T>> types"); 
    }
    fn vec_merge(&self, _vec_2: &Vec<f32>, _action: MergeActions) -> Vec<f32> {
        panic!("Merging vectors is only supported for Vec<T> not Vec<Option<T>> types");
    }
    fn vec_median(&self) -> f32 {
        let mut clean_vec: Vec<f32> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        clean_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = clean_vec.len() / 2;
        let median = clean_vec[mid];
        median
    }

    fn vec_mode(&self) -> f32 {
        let mut clean_vec: Vec<f32> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        
        let mut times = HashMap::new();
        // count
        for x in &clean_vec {
            let cnt = times.entry(*x as usize).or_insert(0.0);
            *cnt += 1.0;
        }

        let mut best: (f32, f32) = (*times.iter().nth(0).expect("Fatal.").0 as f32, *times.iter().nth(0).expect("Fatal.").1 as f32);

        for x in times.iter() {
            if *x.1 > best.1 {
                best = (*x.0 as f32, *x.1);
            }
        }
        best.0
    }
    fn vec_variance(&self) -> f32 {
        panic!("Variance for a vector is only supported for Vec<T> not Vec<Option<T>> types");
    }
    fn vec_std_dev(&self) -> f32 {
        panic!("Standard deviation is only supported for Vec<T> not Vec<Option<T>> types");
    }
    fn to_regtypes(&self, method: VecFillMethod) -> Vec<reg_utils::RegTypes> {
        self.into_iter().map(|x| {
            match x {
                Some(value) => RegTypes::Float(*value),
                None => {
                    match method {
                        VecFillMethod::FillAvg => RegTypes::Float(self.mean_of_vec()),
                        VecFillMethod::FillMedian => RegTypes::Float(self.vec_median()),
                        VecFillMethod::FillMode => RegTypes::Float(self.vec_mode()),
                        VecFillMethod::None => panic!("No method provided for filling in or removing values")  
                }
            }
        }
        }).collect_vec()
    }
    fn vec_max(&self) -> f32 {
        let mut clean_vec: Vec<f32> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        clean_vec.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
    }
    fn vec_min(&self) -> f32 {
        let mut clean_vec: Vec<f32> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        clean_vec.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
    }
}

impl VecActions<i64> for Vec<Option<i64>> {
    fn sum_of_vec(&self) -> i64 {
        let mut clean_vec: Vec<i64> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        } 
        let number = clean_vec.into_iter().map(|v| v).sum::<i64>();
        number
    }
    fn mean_of_vec(&self) -> f32 {
        let mut clean_vec: Vec<i64> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        } 
        let sum = self.sum_of_vec() as f32;
        let count: f32 = clean_vec.len() as f32;
        let mean = sum / count;
        mean
    }
    fn square_vec_values(&self) -> Vec<i64> {
        panic!("Squaring vector values is only supported for Vec<T> not Vec<Option<T>> types"); 
    }
    fn vec_merge(&self, _vec_2: &Vec<i64>, _action: MergeActions) -> Vec<i64> {
        panic!("Merging vectors is only supported for Vec<T> not Vec<Option<T>> types");
    }
    fn vec_median(&self) -> i64 {
        let mut clean_vec: Vec<i64> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        clean_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = clean_vec.len() / 2;
        let median = clean_vec[mid];
        median
    }

    fn vec_mode(&self) -> i64 {
        let mut clean_vec: Vec<i64> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        
        let mut times = HashMap::new();
        // count
        for x in &clean_vec {
            let cnt = times.entry(*x as usize).or_insert(0);
            *cnt += 1;
        }

        let mut best: (i64, i64) = (*times.iter().nth(0).expect("Fatal.").0 as i64, *times.iter().nth(0).expect("Fatal.").1 as i64);

        for x in times.iter() {
            if *x.1 > best.1 {
                best = (*x.0 as i64, *x.1);
            }
        }
        best.0
    }
    fn vec_variance(&self) -> f32 {
        panic!("Variance for a vector is only supported for Vec<T> not Vec<Option<T>> types");
    }
    fn vec_std_dev(&self) -> f32 {
        panic!("Standard deviation is only supported for Vec<T> not Vec<Option<T>> types");
    }
    fn to_regtypes(&self, method: VecFillMethod) -> Vec<reg_utils::RegTypes> {
        self.into_iter().map(|x| {
            match x {
                Some(value) => RegTypes::Int(*value),
                None => {
                    match method {
                        VecFillMethod::FillAvg => RegTypes::Int(self.mean_of_vec().round() as i64),
                        VecFillMethod::FillMedian => RegTypes::Int(self.vec_median()),
                        VecFillMethod::FillMode => RegTypes::Int(self.vec_mode()),
                        VecFillMethod::None => panic!("No method provided for filling in or removing values")  
                }
            }
        }
        }).collect_vec()
    }
    fn vec_max(&self) -> i64 {
        let mut clean_vec: Vec<i64> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        clean_vec.iter().max().unwrap().clone()
    }
    fn vec_min(&self) -> i64 {
        let mut clean_vec: Vec<i64> = Vec::new();
        for i in 0..self.len() {
            match self[i] {
                Some(value) => clean_vec.push(value),
                None => ()
            }
        }
        clean_vec.iter().min().unwrap().clone()
    }
}