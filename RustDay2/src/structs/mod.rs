#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

mod struct_1;

pub fn main() {
    struct_1::struct_1();
}