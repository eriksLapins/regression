pub mod reg_utils;
pub mod vec_actions;
pub mod reg_actions;

use crate::traits::*;
use crate::utils::VecActions;
use itertools::Itertools;
use nalgebra::{DMatrix, Matrix};
pub enum MergeActions {
    Multiply,
    Power,
    Sum,
    Divide,
    Subtract
}

#[derive(Clone, Debug)]
pub struct RegStruct {
    pub name: String,
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

    fn to_matrix_vec(&self, vector: &Vec<Vec<f32>>, width: usize) -> Vec<f32> {
        let mut new_vec = vec![];

        for i in 0..width {
            new_vec.append(& mut vector[i].clone());
        }

        new_vec
    }

    fn into_matrix(&self, vector: &Vec<Vec<f32>>) -> Matrix<f32, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<f32, nalgebra::Dyn, nalgebra::Dyn>> {
        let width = vector.len();
        let height = vector[0].len();
        
        let new_vec = self.to_matrix_vec(vector, width);
        DMatrix::from_vec(height, width, new_vec)
    }

    fn matrix_to_vec(&self, matrix: Matrix<f32, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<f32, nalgebra::Dyn, nalgebra::Dyn>>) -> Vec<Vec<f32>> {
        let height = matrix.nrows();
        let vector = matrix.into_iter().map(|row| row.to_owned()).collect_vec();

        let split: Vec<Vec<f32>> = vector.chunks(height).map(|vec| vec.into()).collect_vec();
        split
    }   
    
    pub fn calc_betas(&self) -> Vec<f32> {
        let width = self.independent_count();
        let height = self.independent[0].vector.len();
        let one_col: Vec<f32> = vec![1.0; height];
        let mut reg_vec = vec![one_col];
        for i in 0..width {
            reg_vec.push(self.independent[i].vector.clone())
        };
        let dependent_vec = self.dependent.vector.clone();
        let y = self.into_matrix(&vec![dependent_vec]);
        let x = self.into_matrix(&reg_vec);
        let xt = x.transpose();
        let xt_x = &xt * &x;
        let xt_x_inverse = xt_x.clone().try_inverse().unwrap();
        let xt_y = &xt * &y;
        let betas = &xt_x_inverse * xt_y;
        let return_betas = self.matrix_to_vec(betas);

        return_betas[0].clone()
    }

    pub fn read_betas(&self) -> () {
        let betas = self.calc_betas();
        for i in 0..betas.len() {
            if i == 0 {
                println!("base {} (b{}): {}", self.dependent.name, i, betas[i]);
            } else {
                println!("{} (b{}): {}", self.independent[i-1].name, i, betas[i])
            }
        }
    }

    pub fn predict(&self, inputs: Vec<f32>) -> std::io::Result<f32> {
        if inputs.len() < self.independent_count {
            panic!("Not enough inputs, expected {}, got {}", self.independent_count, inputs.len());
        } else if inputs.len() > self.independent_count {
            panic!("Too many inputs, expected {}, got {}", self.independent_count, inputs.len());
        } else {
            let betas = self.calc_betas();
            let mut result: f32 = betas[0];
            for i in 1..betas.len() {
                result += inputs[i-1]*betas[i];
            }
    
            Ok(result)
        }
    }
}