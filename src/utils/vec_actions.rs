
use crate::utils::*;
use crate::traits::VecActions;

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
}