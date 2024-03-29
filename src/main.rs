#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{gpio::gpiob, pac, prelude::*};

mod keypad;
mod shift_register;
use keypad::{Key, KeyPad};
use shift_register::ShiftRegister;
use snake_game_stm32_rust::*;

static mut PRESS_FLAG: bool = false;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Hello");

    let dp = pac::Peripherals::take().unwrap();
    let pc = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(8.MHz())
        .freeze(&mut flash.acr);

    let mut delay = pc.SYST.delay(&clocks);

    let mut gpio_a = dp.GPIOA.split();
    let mut gpio_b = dp.GPIOB.split();

    let columns = [
        gpio_a.pa0.into_push_pull_output(&mut gpio_a.crl).erase(),
        gpio_a.pa1.into_push_pull_output(&mut gpio_a.crl).erase(),
        gpio_a.pa2.into_push_pull_output(&mut gpio_a.crl).erase(),
        gpio_a.pa3.into_push_pull_output(&mut gpio_a.crl).erase(),
    ];

    let rows = [
        gpio_a.pa4.into_pull_down_input(&mut gpio_a.crl).erase(),
        gpio_a.pa5.into_pull_down_input(&mut gpio_a.crl).erase(),
        gpio_a.pa6.into_pull_down_input(&mut gpio_a.crl).erase(),
        gpio_a.pa7.into_pull_down_input(&mut gpio_a.crl).erase(),
    ];
    let mut keypad = KeyPad::new(columns, rows);

    let lock_pin = gpio_b.pb12.into_push_pull_output(&mut gpio_b.crh).erase();
    let clock_pin = gpio_b.pb13.into_push_pull_output(&mut gpio_b.crh).erase();
    let data_pin = gpio_b.pb14.into_push_pull_output(&mut gpio_b.crh).erase();

    let mut shift_register = ShiftRegister::new(lock_pin, clock_pin, data_pin);

    let led_arr = [
        LED_1, LED_2, LED_3, LED_4, LED_5, LED_6, LED_7, LED_8, LED_9, LED_10, LED_11, LED_12,
        LED_13, LED_14, LED_15, LED_16,
    ];

    loop {
        let key = keypad.key_pooling();
        unsafe {
            process_key(key);
        }

        // delay.delay_ms(10_u32);
        
        for pin in led_arr {
            shift_register.write_array(&pin);
            delay.delay_ms(100_u32);
        }
    }
}

unsafe fn process_key(key: Option<Key>) {
    match key {
        Some(key) => {
            if !PRESS_FLAG {
                PRESS_FLAG = true;
                rprintln!("{:#?}", key);
            }
        }
        None => {
            if PRESS_FLAG {
                PRESS_FLAG = false;
            }
        }
    }
}
