use embassy_stm32::{Peri, gpio::{AnyPin, Input, Pull}};

use crate::drivers::button::Button;

pub struct RotaryEncoder {
    ch_a: Input<'static>,
    ch_b: Input<'static>,
    button: Button,
    last_state: bool,
}

impl RotaryEncoder {
    pub fn new(pin_a: Peri<'static, AnyPin>, pin_b: Peri<'static, AnyPin>, pin_btn: Peri<'static, AnyPin>) -> Self {
        let ch_a = Input::new(pin_a, Pull::Down);
        let ch_b = Input::new(pin_b, Pull::Down);
        let last_state = ch_a.is_high();
        let button = Button { button: Input::new(pin_btn, Pull::Down) };
        Self { ch_a, ch_b, button, last_state }
    }

    /// Retourne +1 si rotation horaire, -1 si anti-horaire, 0 si rien
    pub fn update(&mut self) -> i8 {
        let a = self.ch_a.is_high();
        let b = self.ch_b.is_high();

        // Détection de front montant sur A
        if a != self.last_state {
            self.last_state = a;

            if a {
                // A vient de passer à HIGH
                if b {
                    return -1; // sens anti-horaire
                } else {
                    return 1; // sens horaire
                }
            }
        }

        0
    }
}
