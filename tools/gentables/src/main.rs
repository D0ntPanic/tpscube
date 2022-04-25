#![feature(box_syntax)]

mod common;
mod cube3x3x3;

fn main() {
    crate::cube3x3x3::TableGenerator::new().generate();
}
