#![no_std]

use stm32f1xx_hal::gpio::{ErasedPin, Output};

pub struct ShiftRegister {
    lock_pin: ErasedPin<Output>,
    clock_pin: ErasedPin<Output>,
    data_pin: ErasedPin<Output>,
}

impl ShiftRegister {
    pub fn new(
        lock_pin: ErasedPin<Output>,
        clock_pin: ErasedPin<Output>,
        data_pin: ErasedPin<Output>,
    ) -> Self {
        Self {
            lock_pin,
            clock_pin,
            data_pin,
        }
    }

    pub fn write_array(&mut self, array: &[u8]) {
        self.lock_pin.set_low();
        for &num in array.iter().rev() {
            self.clock_pin.set_low();
            if num == 0 {
                self.data_pin.set_low();
            } else {
                self.data_pin.set_high();
            }
            self.clock_pin.set_high();
        }
        self.lock_pin.set_high();
    }
}
