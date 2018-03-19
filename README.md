# mma7660fc

A platform agnostic driver to interface with the MMA7660FC 3-Axis Accelerometer via I2C

This chip can be found on Seeed's Grove 3-Axis Digital Accelerometer(±1.5g)

This driver was built using [`embedded-hal`] traits.

[`embedded-hal`]: https://docs.rs/embedded-hal/~0.1

## What works

- Read the XOUT register for value of x
- Read registers XOUT, YOUT and ZOUT to get  6-bits output value for x, y and z
- Set mode to active or standby

## TODO

- [ ] Allow to modify sampling rate, currently initialised to 4 samples per second active
- [ ] Get other data such as orientation, tilt, etc
- [ ] Test with stm32f30x-hal, currently only testing with linux-embedded-hal

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Resources

The following resources were consulted when making this driver:
- https://www.nxp.com/docs/en/data-sheet/MMA7660FC.pdf
- https://github.com/japaric/lsm303dlhc
- https://github.com/therealprof/mag3110
- https://github.com/Seeed-Studio/Accelerometer_MMA7660
- http://pramode.in/2018/02/24/an-introduction-to-writing-embedded-hal-based-drivers-in-rust/
- https://blog.dbrgn.ch/2018/3/13/rust-mcp3425-driver/
