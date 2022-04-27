use std::convert::TryInto;
use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::{collections::HashMap, fs::File};
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Bmp<'a> {
    header: &'a [u8],
    body: &'a [u8],
    header_currentPoint: u32,
    body_currentPoint: u32,
}

#[derive(Debug)]
pub enum BmpErr {
    FileNotFound,
    FaileLoad,
}
enum Header<'a> {
    Name(&'a str),
    Size(u8),
}

#[allow(non_snake_case)]
pub fn loadImgFiletoVec(buf: &mut Vec<u8>, filename: String) -> Result<bool, BmpErr> {
    let mut f = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Err(BmpErr::FileNotFound),
    };

    match f.read_to_end(buf) {
        Ok(_) => (),
        Err(_) => return Err(BmpErr::FaileLoad),
    };
    println!("Complete reading image file");
    Ok(true)
}

impl<'a> Bmp<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Bmp {
            header: &buf[0..54],
            body: &buf[54..],
            header_currentPoint: 0,
            body_currentPoint: 0,
        }
    }
    pub fn write(&self, body: &[u8]) {
        let mut file = fs::File::create("result.bmp").unwrap();
        file.write(self.header).unwrap();
        file.write(body).unwrap();
    }
    pub fn header_read(&mut self, size: u8) -> &[u8] {
        let start = self.header_currentPoint as usize;
        let end = (self.header_currentPoint + (size as u32)) as usize;
        self.header_currentPoint = end as u32;
        &self.header[start..end]
    }
    pub fn body_read(&mut self, size: u32) -> &[u8] {
        let start = self.body_currentPoint as usize;
        let end = (self.body_currentPoint + size) as usize;
        self.body_currentPoint = end as u32;
        &self.body[start..end]
    }

    pub fn get_light(&mut self) -> ([u8; 3], i32) {
        let rgb = self.body_read(3);
        (
            rgb.try_into().unwrap(),
            (rgb[0] as f32 * 0.07 + rgb[1] as f32 * 0.72 + rgb[2] as f32 * 0.21).round() as i32,
        )
    }
    pub fn inspect(&self, offset: i64) -> [u8; 3] {
        let mut rgb: [u8; 3] = [0, 0, 0];
        for j in 0..3 {
            rgb[j] = self.body[((self.body_currentPoint - 3) as i64 + offset + j as i64) as usize];
        }
        rgb
    }
    pub fn inspect_light(&self, offset: i64) -> i32 {
        let mut rgb: [u8; 3] = [0, 0, 0];
        for j in 0..3 {
            rgb[j] = self.body[((self.body_currentPoint - 3) as i64 + offset + j as i64) as usize];
        }
        (rgb[0] as f32 * 0.07 + rgb[1] as f32 * 0.72 + rgb[2] as f32 * 0.21).round() as i32
    }
    pub fn get_header(&mut self, header_info: &mut HashMap<&str, i64>) {
        let header_map = make_header_mapp();

        let format = self.header_read(2);
        println!("format:ox{:x}", format[0]);
        println!("format:ox{:x}", format[1]);
        for elem in &header_map {
            let val = convert_u8(self.header_read(if let Header::Size(s) = &elem[1] {
                *s
            } else {
                panic!("Unecpected Error!");
            }));
            let name = if let Header::Name(n) = &elem[0] {
                n
            } else {
                panic!("Unecpected Error!");
            };
            println!("{}:{}", name, val);
            header_info.insert(name, val);
        }
    }
}
fn convert_u8(target: &[u8]) -> i64 {
    let mut result: i64 = 0;
    let bit8: i64 = 256;
    for (i, elem) in (0_u32..).zip(target.iter()) {
        result += (*elem as i64) * bit8.pow(i);
    }
    result
}

fn make_header_mapp<'a>() -> [[Header<'a>; 2]; 15] {
    let header_names = [
        [Header::Name("Size"), Header::Size(4)],
        [Header::Name("reserveArea1"), Header::Size(2)],
        [Header::Name("reserveArea2"), Header::Size(2)],
        [Header::Name("HeaderSize"), Header::Size(4)],
        [Header::Name("InformationHeaderSize"), Header::Size(4)],
        [Header::Name("width"), Header::Size(4)],
        [Header::Name("height"), Header::Size(4)],
        [Header::Name("plane"), Header::Size(2)],
        [Header::Name("color"), Header::Size(2)],
        [Header::Name("zipFormat"), Header::Size(4)],
        [Header::Name("zipSize"), Header::Size(4)],
        [Header::Name("HorizonalResolution"), Header::Size(4)],
        [Header::Name("VerticalResolution"), Header::Size(4)],
        [Header::Name("colors"), Header::Size(4)],
        [Header::Name("importantColors"), Header::Size(4)],
    ];
    header_names
}
