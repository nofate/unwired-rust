mod gpio;

fn main() {
    let mut mmap = gpio::gpio_setup();
    gpio::gpio_direction(&mut mmap, 27, gpio::Direction::Out);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio::gpio_set(&mut mmap, 27, true);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio::gpio_set(&mut mmap, 27, false);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio::gpio_set(&mut mmap, 27, true);
    std::thread::sleep(std::time::Duration::from_secs(5));
    gpio::gpio_set(&mut mmap, 27, false);
}
