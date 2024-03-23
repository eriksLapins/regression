pub mod traits;
pub mod utils;

use crate::{traits::*, utils::*};

pub fn run_reg() {
    let y: Vec<Option<i64>> = vec![Some(60), Some(62), Some(67), None, Some(71), Some(72), Some(75)];
    let x = vec![140, 155, 159, 179, 192, 200, 212];
    let x_1: Vec<Option<f32>> = vec![Some(70.1), Some(85.2), Some(87.0), Some(90.1), Some(92.0), Some(102.1), Some(105.1)];

    let some_y = RegStruct::new(&y.to_regtypes(VecFillMethod::FillAvg), "height", true);
    let some_x = RegStruct::new(&x.to_regtypes(VecFillMethod::None),  "weight", false);
    let some_x_1 = RegStruct::new(&x_1.to_regtypes(VecFillMethod::FillAvg), "bmi", false);

    // some_y.simple_reg(&some_x);
    // some_y.summarize();
    
    let new_reg_struct = RegVec::new(
        some_y,
        vec![some_x, some_x_1]);
    new_reg_struct.read_betas();
    let inputs = Vec::from([150.0, 80.0]);
    let result = new_reg_struct.predict(inputs).unwrap();
    println!("{result}")
}