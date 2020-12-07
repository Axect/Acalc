extern crate peroxide;
extern crate acalc;
use acalc::*;
use peroxide::fuga::*;

fn main() {
    let df = load_data(2020, Origin);
    let dg = load_data(2020, Standard);

    let mut us = UserScore::with_grade(vec!["KO", "MA1", "WG", "SC"], vec![1,3,3,3], &df, &dg);

    let score = us
        .set_level(Level::Medium)
        .set_weight(vec![40.0, 40.0, 10.0, 10.0])
        .calc_standard()
        .calc_score();
    score.print();

    us.standard.print();
}
