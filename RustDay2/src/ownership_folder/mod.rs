#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

mod ownership;
mod references;
mod slices;

pub fn main() {
    // ownership::ownership();
    // references::references();
    slices::slices();
}