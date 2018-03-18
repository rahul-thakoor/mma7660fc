extern crate linux_embedded_hal as hal;
extern crate mma7660fc;
use std::thread;
use std::time::Duration;

use hal::I2cdev;
use mma7660fc::*;

fn main(){
    let dev = I2cdev::new("/dev/i2c-1").unwrap();

    let mut  acc = Mma7660fc::new(dev).unwrap();

    loop{
        let res = acc.get_x().unwrap();
	    println!("{:x}",res);
        println!("{}",res);

        let mut c =  res & 0x3F;

        if(c > 31){
            c -= 64;
        }
        println!("{}",c);

        thread::sleep(Duration::from_secs(3));
    }



}
