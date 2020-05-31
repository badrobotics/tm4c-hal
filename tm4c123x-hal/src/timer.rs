//! Timers

pub use tm4c123x::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TIMER5};
pub use tm4c123x::{WTIMER0, WTIMER1, WTIMER2, WTIMER3, WTIMER4, WTIMER5};

pub use tm4c_hal::timer_hal_macro;

extern crate embedded_hal as hal;

use hal::timer::CountDown;
use hal::timer::Periodic;

use crate::time::Hertz;
use crate::sysctl::Clocks;
use crate::sysctl;

use void::Void;

/// Hardware timers
pub struct Timer<TIM> {
    tim: TIM,
    clocks: Clocks,
    timeout: Hertz,
}

/// Interrupt events
pub enum Event {
    /// Timer timed out / count down ended
    TimeOut,
}

timer_hal_macro! {
    TIMER0: (timer0, Timer0),
    TIMER1: (timer1, Timer1),
    TIMER2: (timer2, Timer2),
    TIMER3: (timer3, Timer3),
    TIMER4: (timer4, Timer4),
    TIMER5: (timer5, Timer5),

    WTIMER0: (wtimer0, WideTimer0),
    WTIMER1: (wtimer1, WideTimer1),
    WTIMER2: (wtimer2, WideTimer2),
    WTIMER3: (wtimer3, WideTimer3),
    WTIMER4: (wtimer4, WideTimer4),
    WTIMER5: (wtimer5, WideTimer5),
}
