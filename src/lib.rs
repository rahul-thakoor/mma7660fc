#![deny(missing_docs)]
#![deny(warnings)]
#![feature(unsize)]
#![no_std]


extern crate embedded_hal as hal;
use hal::blocking::i2c::{Write, WriteRead};

pub const ADDRESS: u8 = 0x4c;

enum  Register {
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

enum Mode {
    STANDBY = 0x00,
    ACTIVE =0x01
}

impl Mode {
    pub fn bits(&self) -> u8 {
        *self as u8
    }
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}


/// MMA7660FC Driver
pub struct Mma7660fc<I2C> {
    i2c: I2C,
}

impl <I2C, E> Mma7660fc <I2C>
where I2C : WriteRead<Error = E> + Write<Error = E>,

{
    /// Creates a new driver from a I2C peripheral
    pub fn new(i2c: I2C) -> Result<Self, E>{
        let mut mma7660fc = Mma7660fc { i2c };

    }

    /// write to register
    pub fn write_register(&mut self,reg:Register,data:u8)->Result<(), E>{
        self.i2c.write(ADDRESS,&[reg.addr(),data]);
    }

    /// set mode

    pub fn setMode(&mut self, mode:Mode){
        self.write_register(Register::MODE,mode);
    }

    /// get x
    pub fn getX(&mut self) -> Result<i8,E>{
        let mut buffer: [u8];

        self.i2c.write_read(ADDRESS,&[Register::XOUT.addr()],buffer)?;

        Ok((buffer) as i8);


    }

}


