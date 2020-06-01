use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::basic_types::vec3::Color;
use crate::basic_types::vec3::Vec3Traits;
use crate::basic_types::vec3::ColorTraits;

pub struct Screen {
    color_array: Vec<Vec<Color>>,
    image_width: i32,
    image_height: i32
}

pub trait ScreenTraits {
    fn new(color_array: Vec<Vec<Color>>, image_width: i32, image_height: i32) -> Self;
    fn empty_screen(image_width: i32, image_height: i32) -> Self;
    fn color_pixel(&mut self, x: usize, y: usize, rgb_val: Color, samples_per_pixel: i32);
    fn write_to_ppm(&self, filename: String);
}

impl ScreenTraits for Screen {
    fn new(color_array: Vec<Vec<Color>>  , image_width: i32, image_height: i32) -> Self {
        Screen {
            color_array: color_array,
            image_width: image_width,
            image_height: image_height
        }
    }

    fn empty_screen(image_width: i32, image_height: i32) -> Self {
        let color_array = vec![vec![Color::new((0.0, 0.0, 0.0)); image_height as usize]; image_width as usize];
        Screen {
            color_array,
            image_width,
            image_height
        }

    }

    fn color_pixel(&mut self, x: usize, y: usize, rgb_val: Color, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;
        self.color_array[x][y] = (scale * rgb_val).sqrt();
    }

    fn write_to_ppm(&self, filename: String) {
        let path = format!("{}.ppm", filename);
        let path = Path::new(&path);
        let mut file = File::create(path).unwrap();
        writeln!(&mut file, "P3").unwrap();
        writeln!(&mut file, "{} {}", self.image_width, self.image_height).unwrap();
        writeln!(&mut file, "255").unwrap();
        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                writeln!(&mut file, "{}", self.color_array[i as usize][j as usize].write_color()).unwrap();
            }
        }
    
    }
}