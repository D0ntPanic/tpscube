use std::collections::BTreeMap;
use tpscube_core::{Cube3x3x3, CubeFace, LastLayerRandomization, OLLAlgorithm};

fn main() {
    let mut counts = BTreeMap::new();
    for _ in 0..10000000 {
        let cube = Cube3x3x3::random_last_layer(CubeFace::Top, LastLayerRandomization::RandomState);
        *counts
            .entry(OLLAlgorithm::from_cube(&cube.as_faces(), CubeFace::Top))
            .or_insert(0usize) += 1;
    }

    for (oll, count) in counts {
        println!("{:?} {}", oll, count);
    }
}
