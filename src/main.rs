//use std::error::Error;
//use std::fs::File;
use std::io::prelude::*;
//use std::path::Path;


//const GPIO_ADDR: i32 = 0x18040000;
//const GPIO_BLOCK: i32 = 48;
// const PATH_GPIO_EXPORT: &static str = "/sys/class/gpio/export";
const PATH_GPIO_EXPORT: &'static str  = "/sys/class/gpio/export";
const PATH_GPIO_PREFIX: &'static str = "/sys/class/gpio";

#[derive(PartialEq)]
enum Direction {
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

fn gpio_export(gpio: i32) {
    println!("opening: {}", PATH_GPIO_EXPORT);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(PATH_GPIO_EXPORT)
        .unwrap();


    write!(&mut file, "{}", gpio).unwrap();
}

fn gpio_direction(gpio: i32, direction: Direction) {
    let path = format!("{}/gpio{}/direction", PATH_GPIO_PREFIX, gpio);
    println!("opening: {}", path);
    let mut file = std::fs::OpenOptions::new().write(true)
        .open(path)
        .unwrap();

    file.write_all(direction.to_string().as_bytes());
}

fn gpio_set(gpio: i32, value: bool) {
    let path = format!("{}/gpio{}/value", PATH_GPIO_PREFIX, gpio);
    println!("opening: {}", path);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap();
    if value {
        file.write_all(b"1");
    } else {
        file.write_all(b"0");
    }
}

extern crate memmap;
const GPIO_ADDR: usize = 0x18040000;
const GPIO_BLOCK_LEN: usize = 48;

use memmap::{Mmap, Protection};

fn gpio_setup() -> Mmap {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/mem")
        .unwrap();

    return Mmap::open_with_offset(&file,
                                  Protection::ReadWrite,
                                  GPIO_ADDR,
                                  GPIO_BLOCK_LEN).unwrap();
}

fn gpio_direction_ex(mmap: &mut Mmap, gpio: i32, dir: Direction) {
    let mut slice_u8 = unsafe { mmap.as_mut_slice() };
    let mut registers: &mut [u32] = unsafe { std::mem::transmute(slice_u8) };
    let mask: u32 = 1 << gpio;
    if dir == Direction::Out {
        registers[0] |=  mask.to_be();
    } else {
        registers[0] &= !mask.to_be();
    }
}

fn gpio_set_ex(mmap: &mut Mmap, gpio: i32, value: bool) {
    let mut slice_u8 = unsafe { mmap.as_mut_slice() };
    let mut registers: &mut [u32] = unsafe { std::mem::transmute(slice_u8) };
    let mask: u32 = 1 << gpio;
    if value {
        registers[4] = mask.to_be();
    } else {
        registers[3] = mask.to_be();
    }
}

fn main() {
//    gpio_export(27);
//    gpio_direction(27, Direction::Out);
//    gpio_set(27, true);
    //    std::thread::sleep(std::time::Duration::from_secs(5));
//    gpio_set(27, false);
    let mut mmap = gpio_setup();
    gpio_direction_ex(&mut mmap, 27, Direction::Out);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio_set_ex(&mut mmap, 27, true);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio_set_ex(&mut mmap, 27, false);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio_set_ex(&mut mmap, 27, true);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio_set_ex(&mut mmap, 27, false);
}
