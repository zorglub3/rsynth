//! This module implements a reader for the SCL file format. A definition or
//! guide to this format can be found [here](https://www.huygens-fokker.org/scala/scl_format.html).
//! There are many scl files out there and this code has _not_ been tested with
//! more than a handful, so beware.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SclError {
    #[error("Scale error io problem: {0:?}")]
    IOError(#[from] std::io::Error),
    #[error("Way too few lines in scale file: {0}")]
    NotEnoughLines(usize),
    #[error("Line count mismatch. Found {0}, expected {1}")]
    LineCountMismatch(usize, usize),
    #[error("Malformed pitch line: {0}")]
    MalformedPitch(String),
    #[error("Malformed line count: {0}")]
    MalformedLineCount(String),
}

pub struct SclFile {
    #[allow(dead_code)]
    comment: String,
    pitches: Vec<f32>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn first_char(line: &str) -> Option<char> {
    line.chars().next()
}

fn scale_lines<P>(filename: P) -> Result<Vec<String>, SclError>
where
    P: AsRef<Path>,
{
    let mut result: Vec<String> = Vec::new();

    for line in read_lines(filename)? {
        let l = line?.clone();
        match first_char(&l) {
            Some('!') => {}
            Some(_x) => result.push(l),
            None => {}
        }
    }

    Ok(result)
}

fn interpret_pitch(pitch_line: &str) -> Result<f32, SclError> {
    if let Some(first_token) = pitch_line.split_whitespace().next() {
        if let Ok(value) = first_token.parse::<usize>() {
            Ok(value as f32)
        } else if let Ok(value) = first_token.parse::<f32>() {
            Ok(value / 1200.)
        } else if let [a, b] = first_token.split('/').collect::<Vec<_>>()[..] {
            if let (Ok(v1), Ok(v2)) = (a.parse::<usize>(), b.parse::<usize>()) {
                Ok((v1 as f32) / (v2 as f32))
            } else {
                Err(SclError::MalformedPitch(pitch_line.to_string()))
            }
        } else {
            Err(SclError::MalformedPitch(pitch_line.to_string()))
        }
    } else {
        Err(SclError::MalformedPitch(pitch_line.to_string()))
    }
}

impl SclFile {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, SclError> {
        let lines = scale_lines(filename)?;

        if lines.len() < 2 {
            Err(SclError::NotEnoughLines(lines.len()))
        } else {
            let comment = lines[0].clone();
            if let Ok(line_count) = lines[1].parse::<usize>() {
                let mut pitches: Vec<f32> = Vec::new();

                pitches.push(1.);

                for i in 0..(line_count - 1) {
                    pitches.push(interpret_pitch(&lines[i + 2])?);
                }

                Ok(SclFile { comment, pitches })
            } else {
                Err(SclError::MalformedLineCount(lines[1].to_string()))
            }
        }
    }

    pub fn to_pitch_vec(&self, root_note: usize, offset: f32, len: usize) -> Vec<f32> {
        let mut result = Vec::new();
        let l = self.pitches.len() as u32;

        for i in 0..len {
            let p = (i as u32) - (root_note as u32);
            let x = ((p % l) + l) % l;
            let o = p / l;

            result.push((o as f32) + self.pitches[x as usize] + offset);
        }

        result
    }
}
