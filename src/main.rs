use std::{fs, process::Command, error::Error};
use itertools::Itertools;

fn extract_microseconds(output: &str) -> Result<usize, Box<dyn Error>> {
    let out = output.lines().last().unwrap();
    let time = if out.ends_with("ms") {
        out["Time: ".len()..out.len()-2].parse::<usize>()? * 1000
    } else {
        out["Time: ".len()..out.len()-3].parse::<usize>()?
    };
    Ok(time)
}

fn main() {
    println!("Hello, world!");
}
