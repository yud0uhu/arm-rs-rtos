use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    // omitted: rtt initialization
    rprintln!("Hello, world!");
    loop {
        asm::bkpt()
    }
}
