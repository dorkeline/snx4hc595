board agnostic driver for snx4hc595 style shift registers in rust

```rs
use snx4hc595::ShiftRegister;

// example gpio pins and timer from a rp2040 board
let mut timer: Timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
let mut srclr = pins.gpio8.into_push_pull_output();
let mut srclk = pins.gpio6.into_push_pull_output();
let mut rclk = pins.gpio7.into_push_pull_output();
let mut ser = pins.gpio5.into_push_pull_output();

let mut reg = reg::ShiftRegister {
    srclr: &mut srclr,
    srclk: &mut srclk,
    rclk: &mut rclk,
    ser: &mut ser,
};

reg.shift_out(&mut timer, &[0b11001001, 0b11101010]);
```
the functions/struct are generic so the pins only need to implement `embedded_hal::digital::v2::OutputPin` and the timer needs to implement `embedded_hal::blocking::delay::DelayUs`.

the pulse delay is 1us which is far more than needed but it should give a very big safety margin for cheap out-of-spec clones