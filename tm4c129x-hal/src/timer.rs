//! Timers

pub use tm4c129x::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TIMER5, TIMER6, TIMER7};

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
    TIMER6: (timer6, Timer6),
    TIMER7: (timer7, Timer7),
}
