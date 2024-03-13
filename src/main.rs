use regression::{traits::*, utils::*};

fn main() {
    let y: Vec<i64> = Vec::from([60, 62, 67, 70, 71, 72, 75]);
    let x: Vec<i64> = Vec::from([140, 155, 159, 179, 192, 200, 212]);

    let some_y = RegStruct::new(&y, true);
    let some_x = RegStruct::new(&x, false);
    some_y.simple_reg(&some_x);

    some_y.summarize();
}
