#![allow(dead_code, unused_imports)]

use std::{error, fmt};
use std::fs::File;

use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};
use image::{Rgba, GenericImageView}; 
use imageproc::drawing::draw_hollow_rect_mut; 
use imageproc::rect::Rect; 

//Using the following artivcle heavily as a reference point
//Link: https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/

#[derive(Copy, Clone, Debug)]
pub struct BBox { 
    pub x1: f32, 
    pub x2: f32, 
    pub y1: f32, 
    pub y2: f32, 
    pub prob: f32, 
}

#[derive(Debug)]
struct PerceiveError { 
    message: String,
}

impl PerceiveError { 
    fn new(msg: &str) -> Self { 
        PerceiveError{ 
            message: msg.to_string()
        }
    }
} 

impl fmt::Display for PerceiveError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", self.message)
    }
}

impl error::Error for PerceiveError {} 

pub fn faceFinder(input: &mut File) -> Result< Vec<BBox>, Box<dyn error::Error>> {
    let model = include_bytes!("mtcnn_model.pb"); 
    let dark = include_bytes!("dark_orb.png"); 
    let light = include_bytes!("light_orb.png");
    let box1 = BBox{
        x1: 1.0, 
        x2: 1.5, 
        y1: 2.0, 
        y2: 2.5, 
        prob: 0.0,
    }; 
    let box2 = BBox{
        x1: 1.0, 
        x2: 1.5, 
        y1: 2.0, 
        y2: 2.5, 
        prob: 0.0,
    }; 
    let mut xs = vec![box1, box2];
    Ok(xs)
} 

pub fn faceFilter(input: &mut File, fil_ter: &str, faces: Vec<BBox>) -> Result<(), Box<dyn error::Error>> {
    Ok(())
}