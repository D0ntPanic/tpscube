#![feature(box_syntax)]

mod common;
mod solve3x3x3;
mod solve4x4x4;

fn main() {
    crate::solve3x3x3::TableGenerator::new().generate();
    crate::solve4x4x4::TableGenerator::new().generate();
}
