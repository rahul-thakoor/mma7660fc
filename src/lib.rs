#![deny(missing_docs)]
#![deny(warnings)]
#![feature(unsize)]
#![no_std]


extern crate embedded_hal as hal;

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
    PD = 0x0A,


}