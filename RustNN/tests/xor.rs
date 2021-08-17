extern crate nn;
extern crate time;

use nn::{NN, HaltCondition, LearningMode};

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn xor_4layers() {
    init();
    // create examples of the xor function
    let examples = [
        (vec![0f64, 0f64], vec![0f64]),
        (vec![0f64, 1f64], vec![1f64]),
        (vec![1f64, 0f64], vec![1f64]),
        (vec![1f64, 1f64], vec![0f64]),
    ];

    // create a new neural network
    let mut net1 = NN::new(&[2,3,3,1]);

    // train the network
    net1.train(&examples)
        .log_interval(Some(1000))
        .halt_condition( HaltCondition::MSE(0.01) )
        .learning_mode( LearningMode::Incremental )
        .momentum(0.1)
        .go();
}
