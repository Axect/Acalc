#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

use crate::Category::Main;
pub use Subject::*;
pub use ScoreKind::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Subject {
    KO,
    MA1,
    MA2,
    KG,
    WG,
    WH,
    EC,
    SC,
    EA,
    LE,
    EM,
    LP,
    PH1,
    PH2,
    CH1,
    CH2,
    BI1,
    BI2,
    EA1,
    EA2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScoreKind {
    Origin,
    Standard
}

pub fn load_data(n: usize, score_kind: ScoreKind) -> DataFrame {
    match n {
        2020 => {
            match score_kind {
                Origin => {
                    DataFrame::read_nc("data/suneung_2020_org.nc").expect("Can't read org nc")
                },
                Standard => {
                    DataFrame::read_nc("data/suneung_2020_std.nc").expect("Can't read std nc")
                }
            }
        },
        _ => panic!("No data available")
    }
}

// =============================================================================
// Write Suneung Data
// =============================================================================
/// 2020 대학수학능력시험 원점수, 표준점수 자료
/// 
/// * Reference: [Megastudy](https://www.megastudy.net/Entinfo/service_p/rank_cut/jungsi_real.asp)
pub fn suneung_2020(sub: Subject, score: f64) -> f64 {
    // 2020
    // 1) Korean
    let korean_origin = c!(100, 91, 85, 77, 67, 55, 43, 32, 23);
    let korean_standard = c!(140, 131, 125, 117, 107, 95, 83, 72, 64);

    // 2) Math 1 (Science)
    let math1_origin = c!(100, 92, 85, 80, 70, 54, 36, 22, 13);
    let math1_standard = c!(134, 128, 122, 118, 110, 97, 82, 70, 63);

    // 3) Math 2 (Literature)
    let math2_origin = c!(100, 84, 76, 65, 51, 35, 23, 16, 11);
    let math2_standard = c!(149, 135, 128, 118, 106, 92, 81, 75, 71);

    // 4) Korean Geo
    let kg_origin = c!(50, 50, 47, 41, 31, 20, 13, 10, 6);
    let kg_standard = c!(66, 66, 64, 60, 53, 45, 40, 38, 35);

    // 5) World Geo
    let wg_origin = c!(50, 48, 46, 43, 35, 25, 19, 12, 9);
    let wg_standard = c!(65, 64, 62, 60, 54, 46, 41, 36, 33);

    // 6) Word History
    let wh_origin = c!(50, 47, 43, 35, 21, 13, 9, 6);
    let wh_standard = c!(65, 63, 60, 55, 45, 40, 37, 35);

    // 7) Economics
    let eco_origin = c!(50, 45, 40, 34, 27, 18, 11, 10, 7);
    let eco_standard = c!(72, 68, 64, 59, 53, 45, 40, 39, 36);

    // 8) Social Culture
    let socu_origin = c!(50, 47, 44, 40, 35, 24, 15, 9, 7);
    let socu_standard = c!(67, 64, 62, 59, 55, 47, 40, 36, 34);

    // 9) Eastern Asis History
    let ea_origin = c!(50, 50, 45, 39, 31, 22, 15, 11, 7);
    let ea_standard = c!(67, 67, 63, 59, 53, 46, 41, 38, 35);

    // 10) Life and Ethics
    let leth_origin = c!(50, 48, 46, 42, 37, 28, 21, 12, 7);
    let leth_standard = c!(65, 64, 62, 59, 55, 48, 42, 35, 31);

    // 11) Ethics and Morphism
    let eth_origin = c!(50, 50, 46, 40, 25, 16, 10, 7);
    let eth_standard = c!(62, 62, 60, 56, 46, 40, 36, 34);

    // 12) Law & Politics
    let law_origin = c!(50, 47, 45, 40, 32, 21, 13, 9, 5);
    let law_standard = c!(67, 65, 63, 60, 54, 46, 40, 37, 34);

    // 13) Physics 1
    let phy1_origin = c!(50, 47, 45, 41, 36, 27, 17, 12, 7);
    let phy1_standard = c!(66, 64, 62, 59, 55, 48, 40, 36, 32);

    // 14) Physics 2
    let phy2_origin = c!(50, 47, 42, 37, 28, 17, 12, 8, 6);
    let phy2_standard = c!(70, 68, 64, 60, 53, 45, 41, 38, 36);

    // 15) Chemistry 1
    let chem1_origin = c!(50, 47, 43, 40, 37, 30, 21, 14, 9);
    let chem1_standard = c!(67, 64, 61, 58, 55, 49, 41, 35, 31);

    // 16) Chemistry 2
    let chem2_origin = c!(50, 50, 44, 39, 32, 21, 13, 9, 6);
    let chem2_standard = c!(67, 67, 63, 59, 54, 46, 40, 37, 35);

    // 17) Biology 1
    let bio1_origin = c!(50, 48, 44, 39, 33, 24, 15, 12, 8);
    let bio1_standard = c!(67, 66, 63, 59, 54, 47, 40, 37, 34);

    // 18) Biology 2
    let bio2_origin = c!(50, 45, 42, 38, 33, 20, 13, 9, 6);
    let bio2_standard = c!(68, 64, 62, 59, 55, 45, 40, 37, 35);

    // 19) Earth 1
    let earth1_origin = c!(50, 42, 38, 34, 28, 21, 15, 11, 8);
    let earth1_standard = c!(74, 67, 63, 59, 53, 47, 41, 37, 34);

    // 20) Earth 2
    let earth2_origin = c!(50, 48, 44, 41, 33, 22, 14, 10, 7);
    let earth2_standard = c!(66, 65, 62, 60, 54, 46, 40, 37, 35);

    match sub {
        Subject::KO => {
            let fit = fit_main(&korean_origin, &korean_standard);
            fit.calc(score, Category::Main)
        }
        Subject::MA1 => {
            let fit = fit_main(&math1_origin, &math1_standard);
            fit.calc(score, Category::Main)
        }
        Subject::MA2 => {
            let fit = fit_main(&math2_origin, &math2_standard);
            fit.calc(score, Category::Main)
        }
        Subject::KG => {
            let fit = fit_sub(&kg_origin, &kg_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::WG => {
            let fit = fit_sub(&wg_origin, &wg_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::WH => {
            let fit = fit_sub(&wh_origin, &wh_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::EA => {
            let fit = fit_sub(&ea_origin, &ea_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::EC => {
            let fit = fit_sub(&eco_origin, &eco_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::SC => {
            let fit = fit_sub(&socu_origin, &socu_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::LE => {
            let fit = fit_sub(&leth_origin, &leth_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::EM => {
            let fit = fit_sub(&eth_origin, &eth_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::LP => {
            let fit = fit_sub(&law_origin, &law_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::PH1 => {
            let fit = fit_sub(&phy1_origin, &phy1_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::PH2 => {
            let fit = fit_sub(&phy2_origin, &phy2_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::CH1 => {
            let fit = fit_sub(&chem1_origin, &chem1_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::CH2 => {
            let fit = fit_sub(&chem2_origin, &chem2_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::BI1 => {
            let fit = fit_sub(&bio1_origin, &bio1_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::BI2 => {
            let fit = fit_sub(&bio2_origin, &bio2_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::EA1 => {
            let fit = fit_sub(&earth1_origin, &earth1_standard);
            fit.calc(score, Category::Sub)
        }
        Subject::EA2 => {
            let fit = fit_sub(&earth2_origin, &earth2_standard);
            fit.calc(score, Category::Sub)
        }
    }

    //let mut df_origin = DataFrame::with_header(vec![
    //    "KO", "MA1", "MA2", "KG", "WG", "WH", "SC", "EA", "EC", "LE", "EM", "LP", "PH1", "PH2", "CH1",
    //    "CH2", "BI1", "BI2", "EA1", "EA2",
    //]);
    //let mut df_standard = DataFrame::with_header(vec![
    //    "KO", "MA1", "MA2", "KG", "WG", "WH", "SC", "EA", "EC", "LE", "EM", "LP", "PH1", "PH2", "CH1",
    //    "CH2", "BI1", "BI2", "EA1", "EA2",
    //]);
    //df_origin["KO"] = korean_origin;
    //df_origin["MA1"] = math1_origin;
    //df_origin["MA2"] = math2_origin;
    //df_origin["KG"] = kg_origin;
    //df_origin["WG"] = wg_origin;
    //df_origin["WH"] = wh_origin;
    //df_origin["SC"] = socu_origin;
    //df_origin["EA"] = ea_origin;
    //df_origin["EC"] = eco_origin;
    //df_origin["LE"] = leth_origin;
    //df_origin["EM"] = eth_origin;
    //df_origin["LP"] = law_origin;
    //df_origin["PH1"] = phy1_origin;
    //df_origin["PH2"] = phy2_origin;
    //df_origin["CH1"] = chem1_origin;
    //df_origin["CH2"] = chem2_origin;
    //df_origin["BI1"] = bio1_origin;
    //df_origin["BI2"] = bio2_origin;
    //df_origin["EA1"] = earth1_origin;
    //df_origin["EA2"] = earth2_origin;
    //df_standard["KO"] = korean_standard;
    //df_standard["MA1"] = math1_standard;
    //df_standard["MA2"] = math2_standard;
    //df_standard["KG"] = kg_standard;
    //df_standard["WG"] = wg_standard;
    //df_standard["WH"] = wh_standard;
    //df_standard["SC"] = socu_standard;
    //df_standard["EA"] = ea_standard;
    //df_standard["EC"] = eco_standard;
    //df_standard["LE"] = leth_standard;
    //df_standard["EM"] = eth_standard;
    //df_standard["LP"] = law_standard;
    //df_standard["PH1"] = phy1_standard;
    //df_standard["PH2"] = phy2_standard;
    //df_standard["CH1"] = chem1_standard;
    //df_standard["CH2"] = chem2_standard;
    //df_standard["BI1"] = bio1_standard;
    //df_standard["BI2"] = bio2_standard;
    //df_standard["EA1"] = earth1_standard;
    //df_standard["EA2"] = earth2_standard;

    //df_origin
    //    .write_nc("data/suneung_2020_org.nc")
    //    .expect("Can't write origin nc file");
    //df_standard
    //    .write_nc("data/suneung_2020_std.nc")
    //    .expect("Can't write standard nc file");
}

// =============================================================================
// Backend Utils
// =============================================================================

#[derive(Debug, Copy, Clone)]
enum Category {
    Main,
    Sub,
}

#[derive(Debug, Clone, Copy)]
struct StandardScore {
    pub mean: f64,
    pub sigma: f64,
}

impl Printable for StandardScore {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl StandardScore {
    fn calc(&self, original_score: f64, subj: Category) -> f64 {
        match subj {
            Category::Main => 20f64 * (original_score - self.mean) / self.sigma + 100f64,
            Category::Sub => 10f64 * (original_score - self.mean) / self.sigma + 50f64,
        }
    }
}

fn fit_main(score: &Vec<f64>, standard: &Vec<f64>) -> StandardScore {
    let data = score.add_col(standard);
    let mut opt = Optimizer::new(data, standard_score_main);
    let p = opt
        .set_init_param(c!(50, 20))
        .set_max_iter(50)
        .set_method(LevenbergMarquardt)
        .optimize();
    //opt.get_error().print();
    StandardScore {
        mean: p[0],
        sigma: p[1],
    }
}

fn fit_sub(score: &Vec<f64>, standard: &Vec<f64>) -> StandardScore {
    let data = score.add_col(standard);
    let mut opt = Optimizer::new(data, standard_score_sub);
    let p = opt
        .set_init_param(c!(25, 10))
        .set_max_iter(10)
        .set_method(LevenbergMarquardt)
        .optimize();
    //opt.get_error().print();
    StandardScore {
        mean: p[0],
        sigma: p[1],
    }
}

/// Standard Score for main
///
/// # Description
///
/// * x: Original Score
/// * p1: Mean
/// * p2: Std
/// * Output: Standard Score
fn standard_score_main(x: &Vec<f64>, p: Vec<Number>) -> Option<Vec<Number>> {
    Some(
        x.iter()
            .map(|&t| Number::from_f64(t))
            .map(|t| 20f64 * (t - p[0]) / p[1] + 100f64)
            .collect(),
    )
}

/// Standard Score function
///
/// # Description
///
/// * x: Original Score
/// * p1: Mean
/// * p2: Std
/// * Output: Standard Score
fn standard_score_sub(x: &Vec<f64>, p: Vec<Number>) -> Option<Vec<Number>> {
    Some(
        x.iter()
            .map(|&t| Number::from_f64(t))
            .map(|t| 10f64 * (t - p[0]) / p[1] + 50f64)
            .collect(),
    )
}
