use embedded_hal_1 as hal;

pub struct Ds3231<I2C> {
    i2c: I2C,
}

impl<I2C: hal::i2c::I2c> Ds3231<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
    pub fn read(&mut self) -> (u8, u8, u8) {
        fn bcd_to_dec(value: u8) -> u8 { (value >> 4) * 10 + (value & 0x0f) }
        let mut regs = [0u8; 7];
        self.i2c.write_read(0x68, &[0x00], &mut regs).unwrap();
        //let year = 2000 + bcd_to_dec(regs[6]) as i32;
        //let month = time::Month::try_from(bcd_to_dec(regs[5])).unwrap();
        //let day = bcd_to_dec(regs[4]); 
        let hour = bcd_to_dec(regs[2]);
        let minute = bcd_to_dec(regs[1]);
        let second = bcd_to_dec(regs[0] & 0x7f);
        (hour, minute, second)
    }
}
