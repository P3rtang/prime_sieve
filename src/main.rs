#![feature(test, const_trait_impl)]

extern crate test;

use std::{
    fmt::Display,
    sync::{atomic::AtomicBool, Arc},
    thread,
    time::{Duration, Instant},
};

const SIZE: usize = 1000000;

fn main() {
    let mut ran_n_times = 0usize;

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        running_clone
            .clone()
            .store(false, std::sync::atomic::Ordering::SeqCst)
    });

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let mut sieve = Sieve::new();
        sieve.evaluate();
        assert_eq!(sieve.num_primes, 78498);
        ran_n_times += 1;
    }

    println!("{}", ran_n_times);
}

pub struct Sieve {
    bit_array: BitArray,
    index: usize,
    primes: Vec<u32>,
    num_primes: usize,
}

impl Sieve {
    fn new() -> Self {
        let mut this = Self {
            bit_array: BitArray::new(),
            index: 3,
            primes: vec![2],
            num_primes: 1,
        };

        let mut range = 2;
        while SIZE > range {
            this.bit_array.set(range);
            range += 2
        }

        this
    }

    fn next_step(&mut self) -> usize {
        while self.bit_array.get(self.index) && self.index < SIZE {
            self.index += 2;
        }
        self.index
    }

    fn evaluate(&mut self) {
        let mut next_prime = self.next_step();
        while self.index < SIZE {
            // self.primes.push(next_prime as u32);
            self.num_primes += 1;
            let mut range = next_prime;
            while SIZE > range {
                self.bit_array.set(range);
                range += next_prime;
            }
            next_prime = self.next_step();
        }
    }
}

pub struct BitArray {
    data: Box<[u8]>,
}

impl BitArray {
    fn new() -> Self {
        Self {
            data: Box::new([0u8; SIZE]),
        }
    }

    fn get(&self, index: usize) -> bool {
        self.data[index / 8] >> (index % 8) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        self.data[index / 8] = self.data[index / 8] | 1 << (index % 8)
    }
}

#[derive(Debug)]
pub struct BitVec {
    data: Vec<u8>,
}

#[allow(dead_code)]
impl BitVec {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.data.get(index / 8).map(|v| v >> (index % 8) & 1 == 1)
    }

    fn set(&mut self, index: usize) {
        if index > self.data.len() * 8 {
            let expand_size = index / 8 - self.data.len() + 1;
            let mut expansion = vec![0u8; expand_size];
            self.data.append(&mut expansion)
        }
        *self.data.get_mut(index / 8).unwrap() =
            self.data.get(index / 8).unwrap() | 1 << (index % 8)
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.data.iter().rev() {
            write!(f, "{:08b} ", *byte)?;
        }

        return Ok(());
    }
}
