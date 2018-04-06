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

    /// create an instance of the driver
    let mut  acc = Mma7660fc::new(dev).unwrap();

    loop{

        let xyzRaw = acc.get_xyz()?;
        let acceleration = acc.get_acceleration()?;
	  print!("X Raw: ");
          println!("{}",xyzRaw.x);
	  print!("Y Raw: ");
          println!("{}",xyzRaw.y);
	  print!("Z Raw: ");
          println!("{}",xyzRaw.z);
	  println!("");

        print!("X: ");
        println!("{}",acceleration.x);
        print!("Y: ");
        println!("{}",acceleration.y);
        print!("Z: ");
        println!("{}",acceleration.z);
        println!("");

        thread::sleep(Duration::from_secs(1));
    }



}
