use hc_08::Hc08;
use std::thread;

use hc_08::parameters::{role::Role, connectable::IsConnectable};
use windows_serial_embedded_hal::delay::Delay;
use windows_serial_embedded_hal::serial::Serial;
fn main() {
    let s = Serial::open("COM4").unwrap();
    let mut hc08 = Hc08::new(s, Delay);
    println!("{:?}", hc08.query_role());
    println!("{:?}", hc08.query_connectable());
    // let mut hc08 = match hc08.into_peripheral_mode() {
    //     Ok(hc08) => hc08,
    //     Err(_) => panic!(),
    // };
    println!("{:?}", hc08.change_role(Role::Slave));
    println!("{:?}", hc08.change_connectable(IsConnectable(true)));

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
