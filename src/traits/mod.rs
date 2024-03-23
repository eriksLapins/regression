use crate::utils::{reg_utils::RegTypes, MergeActions, RegStruct};
pub trait RegActions {
    fn simple_reg(&self, x_1: &RegStruct) -> (f32, f32);
}

pub enum VecFillMethod {
    None,
    // DeleteRow,
    FillMedian,
    FillAvg,
    // Should only use this if you do not have unique values
    FillMode,
    // BackFill,
    // FrontFill,
}

pub trait VecActions<T> {
    fn vec_sum(&self) -> T;
    fn vec_mean(&self) -> f32;
    fn vec_value_squares(&self) -> Vec<T>;
    fn vec_merge(&self, vec_2: &Vec<T>, action: MergeActions) -> Vec<T>;
    fn vec_median(&self) -> T;
    fn vec_mode(&self) -> T;
    fn vec_variance(&self) -> f32;
    fn vec_std_dev(&self) -> f32;
    fn to_regtypes(&self, method: VecFillMethod) -> Vec<RegTypes>;
    fn vec_max(&self) -> T;
    fn vec_min(&self) -> T;
}

pub trait RegUtils<T> {
    fn new(data: &Vec<T>, dependent: bool) -> Self;
    fn is_dependent(&self) -> bool;
    fn summarize(&self);
}