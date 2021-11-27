use std::thread;
use hc_08::Hc08;

use windows_serial_embedded_hal::delay::Delay;
use windows_serial_embedded_hal::serial::Serial;
fn main() {
    let s = Serial::open("COM4").unwrap();
    let mut hc08 = Hc08::new(s, Delay);
    println!("{:?}", hc08.query_role());
    println!("{:?}", hc08.query_connectable());
    let mut n = 0;
    loop {
        n += 1;
        println!("{}", n);
        hc08.write_buffer("zequan".as_bytes()).unwrap();
        thread::sleep(std::time::Duration::new(1, 0));
    }
}
