extern crate linux_embedded_hal as hal;
extern crate mma7660fc;
extern crate cast;
use std::thread;
use std::time::Duration;
use cast::*;

use hal::I2cdev;
use mma7660fc::*;

fn main(){
    let dev = I2cdev::new("/dev/i2c-1").unwrap();

    let mut  acc = Mma7660fc::new(dev).unwrap();

    loop{
        let res = acc.get_x().unwrap();
        //let xyz = acc.get_xyz().unwrap();
       // println!("Data is");
       // println!("{}",xyz.x);
       // println!("{}",xyz.y);
       // println!("{}",xyz.z);
	println!("{}",res);
        

        thread::sleep(Duration::from_secs(1));
    }



}
