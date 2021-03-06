//! A platform agnostic driver to interface with the MMA7660FC 3-Axis Accelerometer via I2C
//! This chip can be found on Seeed's Grove 3-Axis Digital Accelerometer with range ±1.5g (6-bit, signed)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!
//!
//! # Example : Reading accelerometer values on RPi
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate mma7660fc;
//! extern crate cast;
//! use std::thread;
//! use std::time::Duration;
//! use cast::*;
//!
//! use hal::I2cdev;
//! use mma7660fc::*;
//!
//! fn main(){
//!
 //!    let dev = I2cdev::new("/dev/i2c-1").unwrap();
//!
//!
//!     let mut  acc = Mma7660fc::new(dev).unwrap();
//!
//!     loop{
//!
//!         let xyzRaw = acc.get_xyz().unwrap();
//!         let acceleration = acc.get_acceleration().unwrap();
//!         print!("X Raw: ");
//!         println!("{}",xyzRaw.x);
//!         print!("Y Raw: ");
//!         println!("{}",xyzRaw.y);
//!         print!("Z Raw: ");
//!         println!("{}",xyzRaw.z);
//!         println!("");
//!
//!         print!("X: ");
//!         println!("{}",acceleration.x);
//!         print!("Y: ");
//!         println!("{}",acceleration.y);
//!         print!("Z: ");
//!         println!("{}",acceleration.z);
//!         println!("");
//!
//!         thread::sleep(Duration::from_secs(1));
//!     }
//!
//!
//! }
//!```
//!
//!
//!
//!

#![deny(warnings)]
#![feature(pattern_parentheses)]
#![feature(unsize)]
#![no_std]

extern crate accelerometer;
extern crate embedded_hal as hal;
extern crate cast;

pub use accelerometer::{Accelerometer, I8x3, F32x3};
use hal::blocking::i2c::{Write, WriteRead};
use core::fmt::Debug;
use core::mem;
use cast::u8;
pub const ADDRESS: u8 = 0x4c;
pub const  SENSITIVITY: f32 = 21.33;

#[allow(dead_code)]
#[derive(Copy, Clone)]
/// Register addresses
pub enum  Register {
    XOUT = 0x00,
    YOUT = 0x01,
    ZOUT = 0x02,
    TILT = 0x03,
    SRST = 0x04,
    SPCNT = 0x05,
    INTSU = 0x06,
    MODE = 0x07,
    SR = 0x08,
    PDET = 0x09,
    PD = 0x0A
}


impl Register {
    /// Get register address.
    fn addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Mode {
    STANDBY = 0x00,
    ACTIVE =0x01
}

impl Mode {
    // Get bits
    fn bits(&self) -> u8 {
        *self as u8
    }
}



/// MMA7660FC Driver
pub struct Mma7660fc<I2C> {
    pub i2c: I2C
}

impl <I2C, E> Mma7660fc <I2C>
where I2C : WriteRead<Error = E> + Write<Error = E>,

{
    /// Creates a new driver from a I2C peripheral
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let mut mma7660fc = Mma7660fc { i2c };

        //set active mode

        mma7660fc.write_register(Register::MODE,Mode::ACTIVE.bits())?;

        // set sampling rate to 4 samples/second active

        mma7660fc.write_register(Register::SR,0x04)?;

        Ok(mma7660fc)

    }

    /// write to register
    pub fn write_register(&mut self,reg:Register,data:u8)->Result<(), E>{
        self.i2c.write(ADDRESS,&[reg.addr(),data])
    }

    /// set mode
    pub fn set_mode(&mut self, mode:Mode) -> Result<(), E>{
        self.write_register(Register::MODE,mode.bits())
    }


    /// Returns the raw x,y and z axis from the sensor
    pub fn get_xyz(&mut self) -> Result<I8x3,E>{
        let mut buffer: [u8; 3] = unsafe { mem::uninitialized() };

        self.i2c.write_read(ADDRESS,&[Register::XOUT.addr()],& mut buffer)?;

        let mut raw_x = u8( (buffer[0]) & 0x3F) as i8;
        let mut raw_y= u8( (buffer[1]) & 0x3F) as i8;
        let mut raw_z = u8( (buffer[2]) & 0x3F) as i8;

        //todo refactor duplication
        if raw_x > 31{
            raw_x -= 64;
        }

        if raw_y > 31{
            raw_y -= 64;
        }

        if raw_z > 31{
            raw_z -= 64;
        }

        Ok(I8x3 {
            x: raw_x,
            y: raw_y,
            z: raw_z,

        })


    }
}

impl<I2C, E> Accelerometer<F32x3> for Mma7660fc<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    type Error = E;

    /// Returns the acceleration with range ±1.5g (6-bit, signed)
    fn acceleration(&mut self) -> Result<F32x3, E> {
        let raw = self.get_xyz()?;

        Ok(F32x3{
            x: (raw.x as f32 )/ SENSITIVITY,
            y: (raw.y as f32 ) / SENSITIVITY,
            z: (raw.z as f32)/ SENSITIVITY
        })
    }
}
