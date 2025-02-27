// Copyright 2016 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::slice::Iter;

#[derive(Clone)]
pub struct Map {
    bits: Vec<u64>,
    pub bit_size: usize,
    length: usize,
    padded: bool,
}

#[test]
fn test_map() {
    let mut map = Map::new(4096, 4);
    for i in 0..4096 {
        for j in 0..16 {
            map.set(i, j);
            if map.get(i) != j {
                panic!("Fail");
            }
        }
    }
}

#[test]
fn test_map_odd() {
    for size in 1..16 {
        let mut map = Map::new(64 * 3, size);
        let max = (1 << size) - 1;
        for i in 0..64 * 3 {
            for j in 0..max {
                map.set(i, j);
                if map.get(i) != j {
                    panic!("Index: {} wanted {} and got {}", i, j, map.get(i));
                }
            }
        }
    }
}

impl Map {
    pub fn new(len: usize, size: usize) -> Map {
        let mut map = Map {
            bit_size: size,
            length: len,
            bits: Vec::with_capacity((len * size) / 64),
            padded: false,
        };
        for _ in 0..len { // TODO: Try finding a O(1) solution to this
            map.bits.push(0)
        }
        map
    }

    pub fn from_raw(bits: Vec<u64>, size: usize, padded: bool) -> Map {
        Map {
            length: (bits.len() * 64 + (size - 1)) / size,
            bit_size: size,
            bits,
            padded,
        }
    }

    pub fn resize(&self, size: usize) -> Map {
        let mut n = Map::new(self.length, size);
        for i in 0..self.length {
            n.set(i, self.get(i));
        }
        n
    }

    fn get_bit_offset(&self, i: usize) -> usize {
        let padding = if self.padded {
            i / (64 / self.bit_size) * (64 % self.bit_size)
        } else {
            0
        };
        i * self.bit_size + padding
    }

    pub fn set(&mut self, i: usize, val: usize) {
        let i = self.get_bit_offset(i);
        let pos = i / 64;
        let mask = (1u64 << self.bit_size) - 1;
        let ii = i % 64;
        self.bits[pos] = (self.bits[pos] & !(mask << ii)) | ((val as u64) << ii);
        let pos2 = (i + self.bit_size - 1) / 64;
        if pos2 != pos {
            let used = 64 - ii;
            let rem = self.bit_size - used;
            self.bits[pos2] = self.bits[pos2] >> rem << rem | (val as u64 >> used);
        }
    }

    pub fn get(&self, i: usize) -> usize {
        let i = self.get_bit_offset(i);
        let pos = i / 64;
        let mask = (1 << self.bit_size) - 1;
        let ii = i % 64;
        let pos2 = (i + self.bit_size - 1) / 64;
        if pos2 == pos {
            ((self.bits[pos] >> ii) & mask) as usize
        } else {
            let used = 64 - ii;
            (((self.bits[pos] >> ii) | (self.bits[pos2] << used)) & mask) as usize
        }
    }

    pub fn iter(&self) -> Iter<'_, u64> {
        self.bits.iter()
    }

}
