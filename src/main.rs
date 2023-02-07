use std::env;
// use crate::kzg-1d;
mod kzg1d;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let my_data: Vec<u64> = vec![1,2,3,4,5,6];
    let my_solomon: ReedSolomon = ReedSolomon::default();

    println!("Hello, world!");
}
