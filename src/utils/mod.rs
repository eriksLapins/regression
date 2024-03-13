pub mod reg_utils;
pub mod vec_actions;
use std::collections::HashMap;

use crate::traits::*;
use crate::utils::VecActions;

use itertools::Itertools;
pub enum MergeActions {
    Multiply,
    Power,
    Sum,
    Divide,
    Subtract
}

#[derive(Clone, Debug)]
pub struct RegStruct<T>{
    pub vector: Vec<T>,
    pub squared_vector: Vec<T>,
    pub dependent: bool,
    pub mean: f32,
    pub length: T,
    pub sum: T,
    pub median: T,
    pub mode: T,
    pub variance: f32,
    pub std_dev: f32,
    pub sum_of_squared: T,
    pub min: T,
    pub max: T,
}

impl RegActions<f32> for RegStruct<f32> {
    fn simple_reg(&self, x_1: &RegStruct<f32>) -> (f32, f32) {
        let sum_xy = self.vector.vec_merge(&x_1.vector, MergeActions::Multiply).sum_of_vec();
        let count = self.vector.len() as f32;

        let b_0 = (self.sum*x_1.sum_of_squared - x_1.sum*sum_xy) / (count*x_1.sum_of_squared - x_1.sum.powf(2.0));
        let b_1 = (count*sum_xy - x_1.sum*self.sum) / (count*x_1.sum_of_squared - x_1.sum.powf(2.0));

        println!("Y = {b_0} + {b_1}*X");
        (b_0, b_1)
    }
}

impl RegActions<i64> for RegStruct<i64> {
    fn simple_reg(&self, x_1: &RegStruct<i64>) -> (f32, f32) {
        let sum_xy = self.vector.vec_merge(&x_1.vector, MergeActions::Multiply).sum_of_vec() as f32;
        let count = self.vector.len() as f32;
        let sum_y = self.sum as f32;
        let sum_x = x_1.sum as f32;
        let sum_x_squared = x_1.sum_of_squared as f32;



        let b_0: f32 = (sum_y*sum_x_squared - sum_x*sum_xy) / (count*sum_x_squared - sum_x.powf(2.0));
        let b_1: f32 = (count*sum_xy - sum_x*sum_y) / (count*sum_x_squared - sum_x.powf(2.0));

        println!("Y = {b_0} + {b_1}*X");
        (b_0, b_1)
    }
}