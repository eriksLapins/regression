use crate::utils::{MergeActions, RegStruct};
pub trait RegActions<T> {
    fn simple_reg(&self, x_1: &RegStruct<T>) -> (f32, f32);
}


pub trait VecActions<T> {
    fn sum_of_vec(&self) -> T;
    fn mean_of_vec(&self) -> f32;
    fn square_vec_values(&self) -> Vec<T>;
    fn vec_merge(&self, vec_2: &Vec<T>, action: MergeActions) -> Vec<T>;
    fn vec_median(&self) -> T;
    fn vec_mode(&self) -> T;
    fn vec_variance(&self) -> f32;
    fn vec_std_dev(&self) -> f32;
}

pub trait RegUtils<T> {
    fn new(data: &Vec<T>, dependent: bool) -> Self;
    fn is_dependent(&self) -> bool;
    fn summarize(&self);
}