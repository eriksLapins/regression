pub mod reg_utils;
pub mod vec_actions;
pub mod reg_actions;

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
pub struct RegStruct {
    pub vector: Vec<f32>,
    pub squared_vector: Vec<f32>,
    pub dependent: bool,
    pub mean: f32,
    pub length: f32,
    pub sum: f32,
    pub median: f32,
    pub mode: f32,
    pub variance: f32,
    pub std_dev: f32,
    pub sum_of_squared: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone)]
pub struct RegVec {
    pub dependent: RegStruct,
    pub independent: Vec<RegStruct>,
    independent_count: usize,
}

impl RegVec {
    pub fn new(dependent: RegStruct, independent: Vec<RegStruct>) -> Self {
        let independent_count = independent.len();
        RegVec {
            dependent,
            independent,
            independent_count
        }
    }
    pub fn independent_count(&self) -> usize {
        self.independent_count
    }

    pub fn simple_reg(&self) -> (f32, f32) {
        self.dependent.simple_reg(&self.independent[0])
    }    
}