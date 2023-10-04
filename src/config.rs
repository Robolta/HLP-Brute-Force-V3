#![allow(dead_code)]

pub const STATES: usize = 16;
pub const TARGET: [u8; STATES] = [0; STATES];

pub const DISTINCT: usize = target_distinct();

const fn target_distinct() -> usize {
    let mut groups: [u8; STATES] = [0; STATES];
    let mut i = 0;
    while i < STATES {
        groups[TARGET[i] as usize] += 1;
        i += 1;
    }
    i = 0;

    let mut count = 0;
    while i < STATES {
        if groups[i] > 0 {
            count += 1;
        }
        i += 1;
    }
    count
}

const TWOS_COMP: [u8; 16] = [0, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
const PI: [u8; 16] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];

#[derive(PartialEq)]
pub enum Debug {
    Initial,
    LayerGen,
    LayerPop,
}

pub const DEBUG_OUTPUTS: [Debug; 3] = [Debug::Initial, Debug::LayerGen, Debug::LayerPop];