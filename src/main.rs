use regression::{traits::*, utils::*};

fn main() {
    let y: Vec<i64> = Vec::from([60, 62, 67, 70, 71, 72, 75]);
    let x: Vec<i64> = Vec::from([140, 155, 159, 179, 192, 200, 212]);
    let x_1: Vec<f32> = Vec::from([140.1, 155.2, 159.0, 179.1, 192.0, 200.1, 212.1]);

    let some_y = RegStruct::new(&y.to_regtypes(), true);
    let some_x = RegStruct::new(&x.to_regtypes(), false);
    let some_x_1 = RegStruct::new(&x_1.to_regtypes(), false);

    some_y.simple_reg(&some_x);
    some_y.summarize();
    
    let new_reg_struct = RegVec::new(
        some_y,
        vec![some_x, some_x_1]);
    println!("{:?}", new_reg_struct.simple_reg());
}
