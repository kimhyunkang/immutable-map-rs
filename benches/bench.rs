#![feature(test)]

extern crate immutable_map;
extern crate rand;
extern crate test;

use immutable_map::Map;
use rand::{Rng, IsaacRng};
use test::Bencher;

#[bench]
fn insert(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    let mut map = Map::new();
    let mut v: usize = 0;

    b.iter(|| {
        let k = rng.gen::<u16>() as usize;

        map = map.insert(k, v);

        v += 1;
    })
}

#[bench]
fn get(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    let mut map = Map::new();
    let mut v: usize = 0;

    for _ in 0 .. 10000 {
        let k = rng.gen::<u16>() as usize;
        map = map.insert(k, v);
        v += 1;
    }

    b.iter(|| {
        let k = rng.gen::<u16>() as usize;

        map.get(&k);
    })
}

#[bench]
fn remove(b: &mut Bencher) {
    let input_size = 10000;

    let mut rng = IsaacRng::new_unseeded();
    let mut inputs = Vec::new();
    let mut map = Map::new();
    let mut v = 0usize;

    for _ in 0 .. input_size {
        let k = rng.gen::<u16>() as usize;
        inputs.push(k);
        map = map.insert(k, v);
        v += 1;
    }

    rng.shuffle(&mut inputs);
    let mut idx = 0;

    b.iter(|| {
        let k = inputs[idx];

        if let Some((removed, _)) = map.remove(&k) {
            map = removed;
        }

        idx = (idx + 1) % input_size;
    })
}

#[bench]
fn iter_small(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    let mut map = Map::new();
    let mut v: usize = 0;

    for _ in 0 .. 10 {
        let k = rng.gen::<u16>() as usize;
        map = map.insert(k, v);
        v += 1;
    }

    b.iter(|| {
        map.iter().count();
    })
}

#[bench]
fn iter_large(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    let mut map = Map::new();
    let mut v: usize = 0;

    for _ in 0 .. 1000 {
        let k = rng.gen::<u16>() as usize;
        map = map.insert(k, v);
        v += 1;
    }

    b.iter(|| {
        map.iter().count();
    })
}
