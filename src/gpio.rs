extern crate memmap;

use self::memmap::{Mmap, Protection};
use std::fs::OpenOptions;
use std::mem::transmute;

const PATH_MEMORY: &'static str  = "/dev/mem";
const GPIO_ADDR: usize = 0x18040000;
const GPIO_BLOCK_LEN: usize = 48;


#[derive(PartialEq)]
pub enum Direction {
    Out, In
}

impl Direction {
    fn to_string(&self) -> &'static str  {
        match *self {
            Direction::Out => "out",
            Direction::In => "in"
        }
    }

    fn to_int(&self) -> i32 {
        match *self {
            Direction::Out => 1,
            Direction::In => 0
        }
    }
}


pub fn gpio_setup() -> Mmap {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(PATH_MEMORY)
        .unwrap();

    return Mmap::open_with_offset(&file,
                                  Protection::ReadWrite,
                                  GPIO_ADDR,
                                  GPIO_BLOCK_LEN).unwrap();
}

pub fn gpio_direction(mmap: &mut Mmap, gpio: i32, dir: Direction) {
    let slice_u8 = unsafe { mmap.as_mut_slice() };
    let mut registers: &mut [u32] = unsafe { transmute(slice_u8) };
    let mask: u32 = 1 << gpio;
    if dir == Direction::Out {
        registers[0] |=  mask.to_be();
    } else {
        registers[0] &= !mask.to_be();
    }
}

pub fn gpio_set(mmap: &mut Mmap, gpio: i32, value: bool) {
    let slice_u8 = unsafe { mmap.as_mut_slice() };
    let mut registers: &mut [u32] = unsafe { transmute(slice_u8) };
    let mask: u32 = 1 << gpio;
    if value {
        registers[4] = mask.to_be();
    } else {
        registers[3] = mask.to_be();
    }
}
