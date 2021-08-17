#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use alloc::vec;
    use crate::println;
    use crate::Model;
    
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    
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
    fn lib_mask_test() {
        use core::convert::TryInto;
        use alloc::string::String;
        use alloc::format;

        init();
        let test_data = vec![
            vec![29, 19, 20],
            vec![3, 129, 228],
            vec![191, 238, 23],
            vec![0, 38, 18]
        ];
        let mut model = Model::new(3, 3, 1);
        model.learn_mask(&test_data);
        for y in 0..test_data.len() {
            let mut row_result: Vec<String> = Vec::new();
            for x in 0..test_data[0].len() {
                row_result.push(format!("{}", model.get_mask_value(x.try_into().unwrap(), y.try_into().unwrap())));
            }
            println!("{}", row_result.join(" "));
        }
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
