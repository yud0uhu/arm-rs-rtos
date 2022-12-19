#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use cortex_m_semihosting::hprintln;
    use stm32f4xx_hal::{
        gpio::{Output, PC13},
        pac,
        prelude::*,
        timer::MonoTimerUs,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();

        let mono = ctx.device.TIM2.monotonic_us(&clocks);
        tick::spawn().ok();
        foo::spawn().unwrap();
        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(local = [led])]
    fn tick(ctx: tick::Context) {
        tick::spawn_after(1.secs()).ok();
        ctx.local.led.toggle();
    }

    #[task(priority = 1)]
    fn foo(ctx: foo::Context) {
        hprintln!("baz").unwrap();
    }
}
