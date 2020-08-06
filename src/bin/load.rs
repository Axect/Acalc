extern crate peroxide;
extern crate acalc;
use acalc::*;
use peroxide::fuga::*;

fn main() {
    let df = load_data(2020, Origin);
    let dg = load_data(2020, Standard);
    df.print();
    dg.print();
}