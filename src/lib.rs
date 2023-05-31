#![no_std]

use core::convert::{Infallible, Into};
use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

/// timings are assumed to work with 1us delays between pulses,
/// all the SNx4HC595's i have work with that
/// the timer is not part of the struct because depending on hal implementation the timer is a
/// singleton and it would prevent any other code from using it for as long as the shift register instance is up
pub struct ShiftRegister<'a, E = Infallible> {
    /// clear pin (active low)
    pub srclr: &'a mut dyn OutputPin<Error = E>,

    /// shift register clock (rising edge)
    pub srclk: &'a mut dyn OutputPin<Error = E>,

    /// latch (rising edge)
    pub rclk: &'a mut dyn OutputPin<Error = E>,

    /// serial input pin (active high)
    pub ser: &'a mut dyn OutputPin<Error = E>,
}

impl<'a> ShiftRegister<'a, Infallible> {
    pub fn shift_out(&mut self, timer: &mut impl DelayUs<u8>, data: &[u8]) {
        self.try_shift_out(timer, data).ok();
    }
}

impl<'a, E> ShiftRegister<'a, E> {
    pub fn try_shift_out(&mut self, timer: &mut impl DelayUs<u8>, data: &[u8]) -> Result<(), E> {
        try_shift_out_impl(self.srclr, self.srclk, self.rclk, self.ser, timer, data)
    }
}

pub fn try_shift_out_impl<E>(
    srclr: &mut dyn OutputPin<Error = E>,
    srclk: &mut dyn OutputPin<Error = E>,
    rclk: &mut dyn OutputPin<Error = E>,
    ser: &mut dyn OutputPin<Error = E>,
    timer: &mut impl DelayUs<u8>,
    data: &[u8],
) -> Result<(), E> {
    srclk.set_low()?;
    rclk.set_low()?;

    // erase current contents
    srclr.set_low()?;
    srclk.set_high()?;
    timer.delay_us(1);
    srclk.set_low()?;
    srclr.set_high()?;
    timer.delay_us(1);

    for &byte in data {
        let mut byte = byte;
        for _ in 0..8 {
            let bit = (byte & 0b0000_0001) != 0;

            ser.set_state(bit.into())?;
            timer.delay_us(1);
            srclk.set_high()?;
            timer.delay_us(1);
            srclk.set_low()?;
            ser.set_low()?;

            byte >>= 1;
        }
    }

    timer.delay_us(1);
    rclk.set_high()?;
    timer.delay_us(1);

    Ok(())
}
