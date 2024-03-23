use crate::utils::{reg_utils::RegTypes, MergeActions, RegStruct};
pub trait RegActions {
    fn simple_reg(&self, x_1: &RegStruct) -> (f32, f32);
}

pub enum VecFillMethod {
    None,
    // DeleteRow,
    FillMedian,
    FillAvg,
    FillMode,
    // BackFill,
    // FrontFill,
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
    fn to_regtypes(&self, method: VecFillMethod) -> Vec<RegTypes>;
    fn vec_max(&self) -> T;
    fn vec_min(&self) -> T;
}

pub trait RegUtils<T> {
    fn new(data: &Vec<T>, dependent: bool) -> Self;
    fn is_dependent(&self) -> bool;
    fn summarize(&self);
}