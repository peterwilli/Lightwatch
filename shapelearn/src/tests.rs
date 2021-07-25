#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use alloc::vec;
    use crate::println;
    
    fn grid_to_train_data(grid: &Vec<Vec<f64>>) -> Vec<(Vec<f64>, Vec<f64>)> {
        let mut train_data: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                train_data.push((vec![x as f64, y as f64], vec![grid[y][x] as f64]));
            }
        }
        return train_data;
    }

    #[test]
    fn nn_test() {
        use nn::{NN, HaltCondition};

        let test_data = vec![
            vec![1.0, 0.9, 0.5],
            vec![0.2, 0.4, 0.12],
            vec![0.18, 0.1, 0.21]
        ];

        let examples = grid_to_train_data(&test_data);
        for e in &examples {
            println!("{:?}", e);
        }
        let mut net = NN::new(&[2, 3, 1]);

        net.train(&examples)
            .halt_condition( HaltCondition::Epochs(200000) )
            .log_interval( Some(100) )
            .momentum( 0.1 )
            .rate( 0.3 )
            .go();

        for &(ref inputs, ref outputs) in examples.iter() {
            let results = net.run(inputs);
            let (result, key) = (results[0], outputs[0]);
            println!("Result: {} real: {}", result, key);
        }            
    }

}
