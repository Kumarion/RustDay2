#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

mod ownership_folder;

fn main() {
    // Run our ownership folder
    ownership_folder::main();
}