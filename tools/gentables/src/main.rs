#![feature(box_syntax)]

mod common;
mod solve3x3x3;

fn main() {
    crate::solve3x3x3::TableGenerator::new().generate();
}
