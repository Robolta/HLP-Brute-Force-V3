mod config;
mod loading;
mod search;

use config::DEBUG_OUTPUTS;

use crate::config::TARGET;
use search::layers;

fn main() {
    let debugging = DEBUG_OUTPUTS.contains(&config::Debug::Initial);

    if debugging { println!("Search started for {:?}", TARGET); }

    let mut x = layers::Layers::generate_layers();
    x.populate_children();
}