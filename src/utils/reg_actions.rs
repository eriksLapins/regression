use crate::traits::RegActions;
use crate::utils::{RegStruct, MergeActions, VecActions};

impl RegActions for RegStruct {
    fn simple_reg(&self, x_1: &RegStruct) -> (f32, f32) {
        let sum_xy = self.vector.vec_merge(&x_1.vector, MergeActions::Multiply).vec_sum();
        let count = self.vector.len() as f32;

        let b_0 = (self.sum*x_1.sum_of_squared - x_1.sum*sum_xy) / (count*x_1.sum_of_squared - x_1.sum.powf(2.0));
        let b_1 = (count*sum_xy - x_1.sum*self.sum) / (count*x_1.sum_of_squared - x_1.sum.powf(2.0));

        println!("Y = {b_0} + {b_1}*X");
        (b_0, b_1)
    }
}