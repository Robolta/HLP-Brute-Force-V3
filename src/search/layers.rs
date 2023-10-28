use std::{cmp::{min, max}, num::NonZeroUsize};
use itertools::iproduct;
use std::collections::HashSet;
use lru::LruCache;

use crate::{config::{STATES, DISTINCT, DEBUG_OUTPUTS, self}, loading::ProgressBar};

/// Simulate a redstone comparator
/// 
/// ```
/// back: u8    // The back signal
/// side: u8    // The side signal
/// mode: bool  // The mode of the comparator
/// ```
fn comparator(back: u8, side: u8, mode: bool) -> u8 {
    if side > back {
        return 0;
    } else if mode {
        return back - side;
    }
    back
}

fn distinct(output: [u8; STATES]) -> usize {
    HashSet::from(output).len()
}

/// A single layer circuit
/// 
/// ```
/// output: [u8; STATES]    // The output values
/// notation: String        // Asterisk Notation
/// children: Vec<usize>    // Indices indicating all valid children
/// valid_parent: bool      // Whether the layer is a valid parent (has children)
/// ```
#[derive(Clone)]
pub struct Layer {
    pub output: [u8; STATES],
    pub distinct: usize,
    pub notation: String,
    pub children: Vec<usize>,
    pub valid_parent: bool
}

impl Layer {
    /// Generate a layer provided the state
    /// 
    /// ```
    /// side: u8        // The barrel signal provided to the side of a comparator
    /// side_mode: bool // The mode of the corresponding comparator
    /// back: u8        // The barrel signal provided to the back of a comparator
    /// back_mode: bool // The mode of the corresponding comparator
    /// ```
    fn new(side: u8, side_mode: bool, back: u8, back_mode: bool) -> Self {
        let mut output = [0; STATES];

        for (input, index) in (0..STATES as u8).zip(0..STATES) {
            output[index] = max(
                comparator(input, side, side_mode),
                comparator(back, input, back_mode),
            )
        }

        let side_star = if side_mode { "*" } else { "" };
        let back_star = if back_mode { "*" } else { "" };
        let notation = format!("{side_star}{side},{back_star}{back};");

        Self {
            output,
            distinct: distinct(output),
            notation,
            children: Vec::new(),
            valid_parent: false,
        }
    }

    fn alternate(compare: u8, subtract: u8) -> Self {
        let mut output = [0; STATES];

        for (input, index) in (0..STATES as u8).zip(0..STATES) {
            output[index] = max(
                comparator(compare, input, false),
                comparator(subtract, input, true)
            )
        }

        let notation = format!("");

        Self {
            output,
            distinct: distinct(output),
            notation,
            children: Vec::new(),
            valid_parent: false,
        }
    }

    fn pass(&self, inputs: [u8; STATES]) -> [u8; STATES] {
        inputs.map(|i| self.output[i as usize])
    }
}

pub struct Layers {
    pub layers: Vec<Layer>,
}

impl Layers {
    /// Generate the relevant layer data
    pub fn generate_layers() -> Self {

        // Debugging Output Initialization
        let mut debugger = ProgressBar::new((STATES * STATES * 4) as u64, 24);
        let debugging = DEBUG_OUTPUTS.contains(&config::Debug::LayerGen);

        let mut ignored = HashSet::new();
        let mut ascending = [0; STATES];
        for i in 0..STATES { ascending[i] = i as u8; }
        ignored.insert(ascending);

        let mut layers = Vec::new();
        let states_u8 = STATES as u8;
        for (side_mode, back_mode, side, back) in iproduct!([false, true], [false, true], 0..states_u8, 0..states_u8) {
            
            if debugging { debugger.add(1); }

            let current_layer = Layer::new(side, side_mode, back, back_mode);
            
            if ignored.contains(&current_layer.output) { continue; }
            ignored.insert(current_layer.output);

            if current_layer.distinct < DISTINCT { continue; }

            layers.push(current_layer.clone());
        }

        // Input goes into the side on both comparators.
        // If the comparator states were the same then it'd be useless, so there are always a compare and a subtract/
        for (compare, subtract) in iproduct!(0..states_u8, 0..states_u8) {

            if debugging { debugger.add(1); }

            let current_layer = Layer::alternate(compare, subtract);

            if ignored.contains(&current_layer.output) { continue; }
            ignored.insert(current_layer.output);

            if current_layer.distinct < DISTINCT { continue; }

            layers.push(current_layer.clone());

        }

        if debugging { debugger.results(&format!("All Layers Generated ({})", layers.len())); }

        Self { layers }
    }

    /// Populate each layer with its relevant children
    pub fn populate_children(&mut self) {

        // Debugging Output Initialization
        let mut debugger = ProgressBar::new(self.layers.len().pow(2) as u64, 24);
        let debugging = DEBUG_OUTPUTS.contains(&config::Debug::LayerPop);

        let mut ignored = HashSet::new();
        let mut ascending = [0; STATES];
        for i in 0..STATES { ascending[i] = i as u8; }
        ignored.insert(ascending);

        for layer in &self.layers {
            ignored.insert(layer.output);
        }

        let mut pair_count = 0;

        for (parent_index, parent) in (&self.layers.clone()).iter().enumerate() {
            for (child_index, child) in (&self.layers.clone()).iter().enumerate() {

                let current_output = child.pass(parent.output);
                if ignored.contains(&current_output) { continue }

                // Adding a worst case calculation can potentially avoid the proper function
                let worst = child.distinct - min(child.distinct, STATES - parent.distinct + 1);
                if worst < DISTINCT && distinct(current_output) < DISTINCT { continue; }

                // Moving this after the distinct condition seems to improve performance, so it only inserts successful cases
                ignored.insert(current_output);

                self.layers[parent_index].children.push(child_index);
                self.layers[parent_index].valid_parent = true;
            }

            if debugging {
                pair_count += self.layers[parent_index].children.len();
                debugger.add(self.layers.len() as u64);
            }
        }

        if debugging { debugger.results(&format!("All Pairs Generated ({})", pair_count)); }
    }
    /*
    fn start_search(&self) {
        let mut data_state = [0; STATES];
        for i in 0..STATES {
            data_state[i] = i as u8;
        }

        let mut layer_indices = Vec::new();
        for i in 0..self.layers.len() {
            layer_indices.push(i);
        }

        let mut legality_cache = &mut CacheManager::new();

        for depth in 1..255 {
            let (mut legality_cache, solution) = self.search(data_state, &layer_indices, depth, &mut legality_cache);
            match solution {
                Some(function) => {

                }
                None => continue,
            }
        }
    }

    fn search<'a>(&self, data_state: [u8; STATES], layer_indices: &Vec<usize>, depth: u8, &mut cache_manager: &'a mut CacheManager) -> (&'a mut CacheManager, Option<Vec<&Layer>>) {
        for layer_index in layer_indices {
            let layer = &self.layers[*layer_index];
            let new_data_state = layer.pass(data_state);
            let result;
            (&mut cache_manager, result) = self.search(new_data_state, &layer.children, depth - 1, &mut cache_manager);
            match result {
                Some(mut function) => {
                    function.insert(0, layer);
                    return (&mut cache_manager, Some(function))
                }
                None => continue,
            }
        }

        (&mut cache_manager, None)
    }
    */
}

struct CacheManager {
    legality_cache: LruCache<[u8; STATES], bool>,
}

impl CacheManager {
    fn new() -> Self {
        Self {
            legality_cache: LruCache::new(NonZeroUsize::new(10000).unwrap()),
        }
    }
}