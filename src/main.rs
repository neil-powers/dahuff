/// Huffman encoding
/// 
/// This program encodes a byte stream using the Huffman encoding algorithm.

use std::io;
use std::vec;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");

    let input = "hello worldhello world";
    let input = input.as_bytes().to_vec();

    let frequencies = frequencies(&input);


}



// fn count_byte_frequencies(input: &Vec<u8>) -> [u32; 256] {
//     let mut frequencies = [0; 256];
//     for byte in input {
//         frequencies[*byte as usize] += 1;
//     }
//     frequencies
// }

struct Node {
    frequency: u32,
    byte: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new_leaf(byte: u8, frequency: u32) -> Node {
        Node {
            frequency,
            byte: Some(byte),
            left: None,
            right: None,
        }
    }

    fn new_internal(left: Node, right: Node) -> Node {
        Node {
            frequency: left.frequency + right.frequency,
            byte: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn new_box(n : Node) -> Box<Node> {
        Box::new(n)
    }
}

fn frequencies(input: &Vec<u8>) -> HashMap<u8, u32> {
    let mut frequencies = HashMap::new();
    for byte in input {
        *frequencies.entry(*byte).or_insert(0) += 1;
    }
    frequencies
}

fn print_byte_frequencies(frequencies: &[u32; 256]) {
    for (byte, &frequency) in frequencies.iter().enumerate() {
        if frequency > 0 {
            println!("Byte {:02x} ({}): {}", byte, byte as u8 as char, frequency);
        }
    }
}
