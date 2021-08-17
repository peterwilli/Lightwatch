use nn::{NN, HaltCondition};
use alloc::vec::Vec;
use alloc::vec;
use log::{info, debug};

pub struct Model {
    channels: u8,
    width: u8,
    height: u8,
    net: NN
}

impl Model {
    pub fn new(width: u8, height: u8, channels: u8) -> Self {
        let hidden_layer_amount: u8 = ((width + height) / 2);
        debug!("hidden_layer_amount: {}", hidden_layer_amount);
        return Model {
            channels: channels,
            width: width,
            height: height,
            net: NN::new(&[2, hidden_layer_amount.into(), channels.into()])
        }
    }

    fn grid_to_train_data(grid: &Vec<Vec<u8>>) -> Vec<(Vec<f64>, Vec<f64>)> {
        let mut train_data: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                train_data.push((vec![x as f64, y as f64], vec![(grid[y][x] as f64) / 255.0]));
            }
        }
        return train_data;
    }
    
    pub fn learn_mask(&mut self, mask: &Vec<Vec<u8>>) {
        let examples = Self::grid_to_train_data(mask);
        self.net.train(&examples)
            .halt_condition( HaltCondition::Epochs(400000) )
            .log_interval( Some(1000) )
            .momentum( 0.1 )
            .rate( 0.3 )
            .go();
    }

    pub fn get_mask_value(&self, x: u8, y: u8) -> u8 {
        let results = self.net.run(&[x.into(), y.into()]);
        return (results[0] * 255.0) as u8;
    }
}