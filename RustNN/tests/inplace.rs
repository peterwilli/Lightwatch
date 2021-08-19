extern crate nn;
extern crate time;

use log::debug;
use nn::{HaltCondition, LearningMode, NN};

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn inplace() {
    init();
    let nn = NN::new(&[2, 3, 1]);
    let mut results = nn.make_vec_for_inplace(&[0.0, 0.0]);
    debug!("results: {:?}", results);
    let pixels_amount = 10;
    for i in 0..pixels_amount {
        let results = nn.run_inplace(&[i.into(), i.into()], &mut results);
    }
}
