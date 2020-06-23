use ml_utils::datasets::get_boston_records_from_file;
use rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusty_machine::learning::lin_reg::LinRegressor;
use rusty_machine::linalg::Matrix;
use rusty_machine::linalg::Vector;

use ml_utils::sup_metrics::r_squared_score;
use rusty_machine::analysis::score::neg_mean_squared_error;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use std::vec::Vec;

pub fn run() -> Result<(), Box<dyn Error>> {
    let fl = "data/housing.csv";
    let mut data = get_boston_records_from_file(&fl);
    data.shuffle(&mut thread_rng());
    // separate out to train and test datasets.
    let test_size: f64 = 0.2;
    let test_size: f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    // 分离数据
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    // differentiate the predictors and the targets.
    let boston_x_train: Vec<f64> = train_data
        .iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    let boston_y_train: Vec<f64> = train_data.iter().map(|r| r.into_targets()).collect();
    let boston_x_test: Vec<f64> = test_data
        .iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    let boston_y_test: Vec<f64> = test_data.iter().map(|r| r.into_targets()).collect();
    let boston_x_train = Matrix::new(train_size, 13, boston_x_train);
    let boston_y_train = Vector::new(boston_y_train);
    let boston_x_test = Matrix::new(test_size, 13, boston_y_train);
    let boston_y_test = Matrix::new(test_size, 1, boston_y_test);
    let mut lin_model = LinRegressor::default();
    println!("{:?}", lin_model);
    lin_model.train(&boston_x_train, &boston_y_train);
    let predictions = lin_model.predict(&boston_x_test).unwrap();
    let predictions = Matrix::new(test_size, 1, predictions);
    let acc = neg_mean_squared_error(&predictions, &boston_y_test);
    println!("linear regression error: {:?}", acc);
    println!(
        "linear regression R2 score: {:?}",
        r_squared_score(&boston_y_test.data(), &predictions.data())
    );
    Ok(())
}
